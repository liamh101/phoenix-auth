import {afterEach, describe, expect, test} from "vitest";
import {flushPromises, shallowMount} from "@vue/test-utils";
import {clearMocks, mockIPC} from "@tauri-apps/api/mocks";
import AccountSyncPage from "../../../components/settings/sync/AccountSyncPage.vue";

describe('Sync Log Table', async () => {
    afterEach(() => {
        clearMocks()
    })

    test('New Sync Account', async () => {
        mockIPC((cmd) => {
            if (cmd !== 'get_existing_sync_account') {
                return 'INVALID'
            }

            throw 'Sync Account does not exist'
        })

        const wrapper = shallowMount(AccountSyncPage);

        await flushPromises();

        expect(wrapper.vm.host).toBe('')
        expect(wrapper.vm.username).toBe('')
        expect(wrapper.vm.password).toBe('')
        expect(wrapper.vm.lockdownForm).toBeFalsy()
    });

    test('Existing Sync Account', async () => {
        mockIPC((cmd) => {
            if (cmd !== 'get_existing_sync_account') {
                return 'INVALID'
            }

            return {id: 1, username: 'test', password: 'password', url: 'https://test.com'};
        })

        const wrapper = shallowMount(AccountSyncPage);

        await flushPromises();

        expect(wrapper.vm.host).toBe('https://test.com')
        expect(wrapper.vm.username).toBe('test')
        expect(wrapper.vm.password).toBe('password')
        expect(wrapper.vm.lockdownForm).toBeTruthy()
    });

    test('Create New Account - Invalid Details', async () => {
        mockIPC((cmd, args) => {
            if (cmd === 'get_existing_sync_account') {
                throw 'Sync Account does not exist'
            }

            if (cmd === 'validate_sync_account') {
                if (args.host === 'https://invalid.com' && args.username === 'test' && args.password === 'password') {
                    throw "404 Could not be found";
                }

                throw 'Test Failed!'
            }

            throw 'Invalid Command'
        })

        const wrapper = shallowMount(AccountSyncPage);

        await flushPromises();

        wrapper.vm.host = 'https://invalid.com'
        wrapper.vm.username = 'test'
        wrapper.vm.password = 'password'

        wrapper.vm.submitForm()

        expect(wrapper.vm.loading).toBeTruthy();

        await flushPromises();

        expect(wrapper.vm.message).toBe('404 Could not be found');
        expect(wrapper.vm.loading).toBeFalsy();
    })

    test('Create New Account - Valid Details', async () => {
        mockIPC((cmd, args) => {
            if (cmd === 'get_existing_sync_account') {
                throw 'Sync Account does not exist'
            }

            if (cmd === 'validate_sync_account') {
                if (args.host === 'https://valid.com' && args.username === 'test' && args.password === 'password') {
                    return "28uwiofdjger3q9wfdghs";
                }

                throw 'Test Failed!'
            }

            if (cmd === 'save_sync_account') {
                if (args.host === 'https://valid.com' && args.username === 'test' && args.password === 'password') {
                    return {id: 1, username: 'test', password: 'password', url: 'https://valid.com'}
                }

                throw 'Invalid account provided'
            }

            throw 'Invalid Command'
        })

        const wrapper = shallowMount(AccountSyncPage);

        await flushPromises();

        wrapper.vm.host = 'https://valid.com'
        wrapper.vm.username = 'test'
        wrapper.vm.password = 'password'

        wrapper.vm.submitForm()

        expect(wrapper.vm.loading).toBeTruthy();

        await flushPromises();

        expect(wrapper.vm.message).toBe('Successfully Validated Account');
        expect(wrapper.vm.loading).toBeFalsy();
        expect(wrapper.vm.lockdownForm).toBeTruthy();
        expect(wrapper.vm.host).toBe('https://valid.com')
        expect(wrapper.vm.username).toBe('test')
        expect(wrapper.vm.password).toBe('password')
    })

    test('Update Existing Account', async () => {
        mockIPC((cmd, args) => {
            if (cmd === 'get_existing_sync_account') {
                return {id: 1, username: 'test', password: 'password', url: 'https://test.com'};
            }

            if (cmd === 'validate_sync_account') {
                if (args.host === 'https://updated.com' && args.username === 'test' && args.password === 'password') {
                    return "28uwiofdjger3q9wfdghs";
                }

                throw 'Test Failed!'
            }

            if (cmd === 'save_sync_account') {
                if (args.host === 'https://updated.com' && args.username === 'test' && args.password === 'password') {
                    return {id: 1, username: 'test', password: 'password', url: 'https://valid.com'}
                }

                throw 'Invalid account provided'
            }

            throw 'Invalid Command'
        })

        const wrapper = shallowMount(AccountSyncPage);

        await flushPromises();

        expect(wrapper.vm.lockdownForm).toBeTruthy();

        wrapper.vm.submitForm()

        expect(wrapper.vm.lockdownForm).toBeFalsy();

        wrapper.vm.host = 'https://updated.com'

        wrapper.vm.submitForm()

        expect(wrapper.vm.loading).toBeTruthy();

        await flushPromises();

        expect(wrapper.vm.message).toBe('Successfully Validated Account');
        expect(wrapper.vm.loading).toBeFalsy();
        expect(wrapper.vm.lockdownForm).toBeTruthy();
        expect(wrapper.vm.host).toBe('https://updated.com')
        expect(wrapper.vm.username).toBe('test')
        expect(wrapper.vm.password).toBe('password')
    });
});