import {afterEach, describe, expect, test} from "vitest";
import {clearMocks, mockIPC} from "@tauri-apps/api/mocks";
import {flushPromises, shallowMount} from "@vue/test-utils";
import AccountList from "../components/AccountList.vue";


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

        const wrapper = shallowMount(
            AccountList,
            {
                propsData: {
                    accountId: 12
                }
            }
        )

        await flushPromises();

        expect(wrapper.html()).toContain('<td>1</td>\n      <td>Account One</td>')
        expect(wrapper.html()).toContain('<td>2</td>\n      <td>Account Two</td>')
        expect(wrapper.html()).not.toContain('<td colspan="3">No accounts added</td>')
    })

    test('No results', async () => {
        mockIPC((cmd) => {
            if (cmd !== 'get_all_accounts') {
                return 'INVALID'
            }

            return '[]';
        })

        const wrapper = shallowMount(
            AccountList,
            {
                propsData: {
                    accountId: 12
                }
            }
        )

        await flushPromises();

        expect(wrapper.html()).toContain('<td colspan="3">No accounts added</td>')
    })
})