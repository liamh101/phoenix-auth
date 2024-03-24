import {afterEach, beforeEach, describe, expect, test} from "vitest";
import {flushPromises, shallowMount} from "@vue/test-utils";
import DeleteAccount from "../../components/accounts/DeleteAccount.vue";
import {clearMocks, mockIPC} from "@tauri-apps/api/mocks";

describe('Delete Account', async () => {
    beforeEach(() => {
        mockIPC((cmd, args) => {
            if (cmd !== "delete_account") {
                return 'INVALID'
            }

            if (args.accountId === 12) {
                return "Success";
            }

            return "Fail";
        })
    })

    afterEach(() => {
        clearMocks()
    })

    test('Successful Removal', async () => {
        const wrapper = shallowMount(
            DeleteAccount,
            {
                props: {
                    accountId: 12,
                }
            }
        )

        wrapper.vm.confirmDelete()
        await flushPromises();
        const emit = wrapper.emitted('success')

        expect(emit).toHaveLength(1)
    })

    test('Failed Removal', async () => {
        const wrapper = shallowMount(
            DeleteAccount,
            {
                props: {
                    accountId: 13,
                }
            }
        )

        wrapper.vm.confirmDelete()
        await flushPromises();
        const emit = wrapper.emitted('success')

        expect(emit).toBeUndefined()
    })
})