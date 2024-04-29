import {afterEach, describe, expect, test} from "vitest";
import {flushPromises, mount} from "@vue/test-utils";
import ImportSelector from "../../../components/settings/imports/ImportSelector.vue";
import {clearMocks, mockIPC} from "@tauri-apps/api/mocks";

describe('Import Selector', async () => {
    afterEach(() => {
        clearMocks()
    })

    test('Default Hides Input', async () => {
        const wrapper = mount(ImportSelector)

        expect(wrapper.html()).not.toContain('<input')
        expect(wrapper.vm.accept).toBe('')
    })

    test('Import WA File', async () => {
        mockIPC((cmd, args) => {
            if (cmd !== "parse_otp_url") {
                return 'INVALID'
            }

            if (args.otpUrl === 'otpauth://totp/Hello%20World?digits=8&secret=SHFLJASF3213') {
                return '{"id": 0, "name": "Hello World", "secret": "SHFLJASF3213", "otp_digits": 8, "totp_step": 0}';
            }

            if (args.otpUrl === 'otpauth://totp/Hello%20World%202?digits=6&secret=Gregg') {
                return '{"id": 0, "name": "Hello World 2", "secret": "Gregg", "otp_digits": 6, "totp_step": 30}';
            }

            return "Fail";
        })

        const wrapper = mount(ImportSelector)

        wrapper.vm.importWaFile('otpauth://totp/Hello%20World?digits=8&secret=SHFLJASF3213\notpauth://totp/Hello%20World%202?digits=6&secret=Gregg\nInvalidString')
        await flushPromises();

        expect(wrapper.emitted().importedAccounts).toBeTruthy()
        expect(wrapper.emitted().importedAccounts.length).toBe(1)
        expect(wrapper.emitted().importedAccounts[0][0].length).toBe(2)
        expect(wrapper.emitted().importedAccounts[0][0][0]).toEqual({"id": 0, "name": "Hello World", "secret": "SHFLJASF3213", "otp_digits": 8, "totp_step": 0, "import": true})
        expect(wrapper.emitted().importedAccounts[0][0][1]).toEqual({"id": 0, "name": "Hello World 2", "secret": "Gregg", "otp_digits": 6, "totp_step": 30, "import": true})
    })
})