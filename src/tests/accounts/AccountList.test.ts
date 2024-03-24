import {afterEach, describe, expect, test} from "vitest";
import {clearMocks, mockIPC} from "@tauri-apps/api/mocks";
import {flushPromises, shallowMount} from "@vue/test-utils";
import AccountList from "../../components/accounts/AccountList.vue";


describe('Display Results', async () => {
    afterEach(() => {
        clearMocks()
    })

    test('Multiple results', async () => {
        mockIPC((cmd) => {
            if (cmd !== 'get_all_accounts') {
                return 'INVALID'
            }

            return '[{"id": 1, "name": "Account One", "secret": null}, {"id": 2, "name": "Account Two", "secret": null}]';
        })

        const wrapper = shallowMount(AccountList)

        await flushPromises();

        expect(wrapper.html()).toContain('<account-item-stub accountid="1" accountname="Account One" manage="false"></account-item-stub>')
        expect(wrapper.html()).toContain('<account-item-stub accountid="2" accountname="Account Two" manage="false"></account-item-stub>')
        expect(wrapper.html()).not.toContain('<h2 class="text-center">No accounts found</h2>')
    })

    test('Manage results', async () => {
        mockIPC((cmd) => {
            if (cmd !== 'get_all_accounts') {
                return 'INVALID'
            }

            return '[{"id": 1, "name": "Account One", "secret": null}, {"id": 2, "name": "Account Two", "secret": null}]';
        })

        const wrapper = shallowMount(
            AccountList,
            {
                props: {
                    manage: true
                }
            }
        )

        await flushPromises();

        expect(wrapper.html()).toContain('<account-item-stub accountid="1" accountname="Account One" manage="true"></account-item-stub>')
        expect(wrapper.html()).toContain('<account-item-stub accountid="2" accountname="Account Two" manage="true"></account-item-stub>')
        expect(wrapper.html()).not.toContain('<h2 class="text-center">No accounts found</h2>')
    })

    test('No results', async () => {
        mockIPC((cmd) => {
            if (cmd !== 'get_all_accounts') {
                return 'INVALID'
            }

            return '[]';
        })

        const wrapper = shallowMount(AccountList)

        await flushPromises();

        expect(wrapper.html()).toContain('No accounts found')
    })
})