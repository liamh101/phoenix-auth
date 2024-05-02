import {describe, expect, test} from "vitest";
import {mount} from "@vue/test-utils";
import ImportResultMessage from "../../../components/settings/imports/ImportResultMessage.vue";

describe('Import Result Message', async () => {
    test('All Imports Successful', async () => {
        const wrapper = mount(
            ImportResultMessage,
            {
                props: {
                    importDetails: {
                        failed: 0,
                        attempted: 10,
                    }
                }
            }
            )

        expect(wrapper.vm.importMessage).toBe('All Accounts Imported Successfully')
    })

    test('Some Failures', async () => {
        const wrapper = mount(
            ImportResultMessage,
            {
                props: {
                    importDetails: {
                        failed: 4,
                        attempted: 10,
                    }
                }
            }
        )

        expect(wrapper.vm.importMessage).toBe('4/10 Failed To Import')
    })

    test('All Failed', async () => {
        const wrapper = mount(
            ImportResultMessage,
            {
                props: {
                    importDetails: {
                        failed: 10,
                        attempted: 10,
                    }
                }
            }
        )

        expect(wrapper.vm.importMessage).toBe('All Accounts Failed To Import')
    })
})