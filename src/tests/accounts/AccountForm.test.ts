import {expect, test, afterEach, describe} from 'vitest'
import { mockIPC, clearMocks } from '@tauri-apps/api/mocks'
import { flushPromises, mount} from '@vue/test-utils';
import AccountForm from "../../components/accounts/AccountForm.vue";
import {AccountAlgorithm} from "../../composables/Commands";


describe('Validation Tests', async () => {
    test('Disable Submit button, no name', async () => {
        const wrapper = mount(AccountForm);
        wrapper.vm.secret = 'Hello'

        expect(wrapper.vm.shouldDisable()).toBeTruthy()
    })

    test('Disable Submit button, no secret', async () => {
        const wrapper = mount(AccountForm);
        wrapper.vm.name = 'Hello'

        expect(wrapper.vm.shouldDisable()).toBeTruthy()
    })

    test('Disable Submit button, both', async () => {
        const wrapper = mount(AccountForm);

        expect(wrapper.vm.shouldDisable()).toBeTruthy()
    })

    test('Enable Submit button', async () => {
        const wrapper = mount(AccountForm)
        wrapper.vm.name = 'Hello'
        wrapper.vm.secret = 'World'

        expect(wrapper.vm.shouldDisable()).toBeFalsy()
    })

    test('Edit Ignores Secret Validation', async () => {
        mockIPC((cmd) => {
            if (cmd !== 'get_editable_account') {
                return 'INVALID'
            }

            return JSON.stringify({id: 1, name: "Hello World", secret: '', otp_digits: 6, totp_step: 60, algorithm: "SHA512"})
        })

        const wrapper = mount(AccountForm, {props: {accountId: 1}})
        wrapper.vm.name = 'Hello'
        wrapper.vm.secret = ''

        expect(wrapper.vm.shouldDisable()).toBeFalsy()
    })
})

describe('Endpoint handling', async () => {
    afterEach(() => {
        clearMocks()
    })

    test('Duplicate Name', async () => {
        mockIPC((cmd, args) => {
            if (cmd !== 'create_new_account') {
                return 'INVALID'
            }

            if (args.name === 'Hello' && args.secret === 'World') {
                return 'Account already exists';
            }

            return '';
        })

        const wrapper = mount(AccountForm)
        wrapper.vm.name = 'Hello'
        wrapper.vm.secret = 'World'

        wrapper.vm.submitForm()

        await flushPromises()

        expect(wrapper.vm.message).toBe('Account already exists')
    })

    test('Invalid 2FA', async () => {
        mockIPC((cmd, args) => {
            if (cmd !== 'create_new_account') {
                return 'INVALID'
            }

            if (args.name === 'Hello' && args.secret === 'World') {
                return 'Invalid 2FA Secret';
            }

            return '';
        })

        const wrapper = mount(AccountForm)
        wrapper.vm.name = 'Hello'
        wrapper.vm.secret = 'World'

        wrapper.vm.submitForm()

        await flushPromises()

        expect(wrapper.vm.message).toBe('Invalid 2FA Secret')
    })

    test('Successful New', async () => {
        mockIPC((cmd, args) => {
            if (cmd !== 'create_new_account') {
                return 'INVALID'
            }

            if (args.name === 'Hello' && args.secret === 'World') {
                return 'Added Successfully';
            }

            return '';
        })

        const wrapper = mount(AccountForm)
        wrapper.vm.name = 'Hello'
        wrapper.vm.secret = 'World'

        wrapper.vm.submitForm()

        await flushPromises()

        expect(wrapper.vm.message).toBe('Added Successfully')
    })

    test('Successful Get Existing Account', async () => {
        mockIPC((cmd) => {
            if (cmd !== 'get_editable_account') {
                return 'INVALID'
            }

            return JSON.stringify({id: 1, name: "Hello World", secret: "encrypted", otp_digits: 6, totp_step: 60, algorithm: "SHA512"})
        })

        const wrapper = mount(AccountForm, {props: {accountId: 1}})
        await flushPromises()

        expect(wrapper.vm.name).toBe('Hello World')
        expect(wrapper.vm.secret).toBe('')
        expect(wrapper.vm.digits).toBe(6)
        expect(wrapper.vm.timestep).toBe(60)
        expect(wrapper.vm.algorithm).toBe(AccountAlgorithm.SHA512)
    })

    test('Successful Update Existing Account', async () => {
        mockIPC((cmd, args) => {
            if (cmd === 'get_editable_account' ) {
                return JSON.stringify({id: 1, name: "Hello World", secret: "encrypted", otp_digits: 6, totp_step: 60, algorithm: "SHA512"})
            }

            if  (cmd === 'edit_account' &&
                args.id === 1 &&
                args.name === "Hello World Edit" &&
                args.digits === 6 &&
                args.step === 60 &&
                args.algorithm === "SHA512"
            ) {
                return 'Updated Account';
            }

            return 'INVALID'
        })

        const wrapper = mount(AccountForm, {props: {accountId: 1}})
        await flushPromises()

        wrapper.vm.name = 'Hello World Edit'

        wrapper.vm.submitForm()

        await flushPromises()

        expect(wrapper.vm.message).toBe('Updated Account')
    })
})