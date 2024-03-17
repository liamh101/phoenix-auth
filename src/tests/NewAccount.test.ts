import {expect, test, afterEach, describe} from 'vitest'
import { mockIPC, clearMocks } from '@tauri-apps/api/mocks'
import { flushPromises, mount} from '@vue/test-utils';
import NewAccount from '../components/accounts/NewAccount.vue';


describe('Validation Tests', async () => {
    test('Disable Submit button, no name', async () => {
        const wrapper = mount(NewAccount);
        wrapper.vm.secret = 'Hello'

        expect(wrapper.vm.shouldDisable()).toBeTruthy()
    })

    test('Disable Submit button, no secret', async () => {
        const wrapper = mount(NewAccount);
        wrapper.vm.name = 'Hello'

        expect(wrapper.vm.shouldDisable()).toBeTruthy()
    })

    test('Disable Submit button, both', async () => {
        const wrapper = mount(NewAccount);

        expect(wrapper.vm.shouldDisable()).toBeTruthy()
    })

    test('Enable Submit button', async () => {
        const wrapper = mount(NewAccount)
        wrapper.vm.name = 'Hello'
        wrapper.vm.secret = 'World'

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

        const wrapper = mount(NewAccount)
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

        const wrapper = mount(NewAccount)
        wrapper.vm.name = 'Hello'
        wrapper.vm.secret = 'World'

        wrapper.vm.submitForm()

        await flushPromises()

        expect(wrapper.vm.message).toBe('Invalid 2FA Secret')
    })

    test('Successful', async () => {
        mockIPC((cmd, args) => {
            if (cmd !== 'create_new_account') {
                return 'INVALID'
            }

            if (args.name === 'Hello' && args.secret === 'World') {
                return 'Added Successfully';
            }

            return '';
        })

        const wrapper = mount(NewAccount)
        wrapper.vm.name = 'Hello'
        wrapper.vm.secret = 'World'

        wrapper.vm.submitForm()

        await flushPromises()

        expect(wrapper.vm.message).toBe('Added Successfully')
    })
})