import {afterEach, describe, expect, test} from "vitest";
import ImportTable from "../../../components/settings/imports/ImportTable.vue";
import {flushPromises, mount} from "@vue/test-utils";
import {nextTick} from "vue";
import {clearMocks, mockIPC} from "@tauri-apps/api/mocks";

describe('Import Table', async () => {
    afterEach(() => {
        clearMocks()
    })

    test('Display Accounts', async () => {
        const accounts = [
            {
                id: 0,
                name: 'Hello World One',
                secret: 'Testing',
                otp_digits: 8,
                totp_step: 30,
                import: true,
            },
            {
                id: 0,
                name: 'Hello World Two',
                secret: 'Testing',
                otp_digits: 8,
                totp_step: 30,
                import: true,
            },
            {
                id: 0,
                name: 'Hello World Three',
                secret: 'Testing',
                otp_digits: 8,
                totp_step: 30,
                import: true,
            }
        ]

        const wrapper = mount(
            ImportTable,
            {
                props: {
                    accounts: accounts
                },
            }
        );

        wrapper.vm.cloneAccounts()
        expect(wrapper.vm.draftAccounts).toStrictEqual(accounts)

        await nextTick()
        expect(wrapper.html()).toContain('<td><span>Hello World One</span>')
        expect(wrapper.html()).toContain('<td><span>Hello World Two</span>')
        expect(wrapper.html()).toContain('<td><span>Hello World Three</span>')
    })

    test('Confirm Import', async () => {
        mockIPC((cmd, args) => {
            if (cmd !== "create_new_account") {
                return 'INVALID'
            }

            if (args.name === 'Hello World One' && args.secret === 'Testing' && args.digits === 8 && args.step === 30) {
                return 'Success';
            }

            if (args.name === 'Hello World Two' && args.secret === 'Testing' && args.digits === 8 && args.step === 30) {
                return 'Success';
            }

            return false;
        })

        const accounts = [
            {
                id: 0,
                name: 'Hello World One',
                secret: 'Testing',
                otp_digits: 8,
                totp_step: 30,
                import: true,
            },
            {
                id: 0,
                name: 'Hello World Two',
                secret: 'Testing',
                otp_digits: 8,
                totp_step: 30,
                import: true,
            },
            {
                id: 0,
                name: 'Hello World Three',
                secret: 'Testing',
                otp_digits: 8,
                totp_step: 30,
                import: false,
            }
        ]

        const wrapper = mount(
            ImportTable,
            {
                props: {
                    accounts: accounts
                },
            }
        );

        wrapper.vm.cloneAccounts()
        await nextTick()

        wrapper.vm.confirmAccounts()
        await flushPromises()

        expect(wrapper.emitted().complete).toBeTruthy()
        expect(wrapper.emitted().complete.length).toBe(1)
        expect(wrapper.emitted().complete[0][0].failed).toBe(0)
        expect(wrapper.emitted().complete[0][0].attempted).toBe(2)
    })

    test('Failed Import', async () => {
        mockIPC((cmd, args) => {
            if (cmd !== "create_new_account") {
                return 'INVALID'
            }

            if (args.name === 'Hello World One' && args.secret === 'Testing' && args.digits === 8 && args.step === 30) {
                return 'Success';
            }

            return false;
        })

        const accounts = [
            {
                id: 0,
                name: 'Hello World One',
                secret: 'Testing',
                otp_digits: 8,
                totp_step: 30,
                import: true,
            },
            {
                id: 0,
                name: 'Hello World Two',
                secret: 'Testing',
                otp_digits: 8,
                totp_step: 30,
                import: true,
            },
            {
                id: 0,
                name: 'Hello World Three',
                secret: 'Testing',
                otp_digits: 8,
                totp_step: 30,
                import: false,
            }
        ]

        const wrapper = mount(
            ImportTable,
            {
                props: {
                    accounts: accounts
                },
            }
        );

        wrapper.vm.cloneAccounts()
        await nextTick()

        wrapper.vm.confirmAccounts()
        await flushPromises()

        expect(wrapper.emitted().complete).toBeTruthy()
        expect(wrapper.emitted().complete.length).toBe(1)
        expect(wrapper.emitted().complete[0][0].failed).toBe(1)
        expect(wrapper.emitted().complete[0][0].attempted).toBe(2)
    })

    test('Display Editor', async () => {
        const accounts = [
            {
                id: 0,
                name: 'Hello World One',
                secret: 'Testing',
                otp_digits: 8,
                totp_step: 30,
                import: true,
            },
            {
                id: 0,
                name: 'Hello World Two',
                secret: 'Testing',
                otp_digits: 8,
                totp_step: 30,
                import: true,
            },
            {
                id: 0,
                name: 'Hello World Three',
                secret: 'Testing',
                otp_digits: 8,
                totp_step: 30,
                import: true,
            }
        ]

        const wrapper = mount(
            ImportTable,
            {
                props: {
                    accounts: accounts
                },
            }
        );

        wrapper.vm.cloneAccounts()
        await nextTick()

        wrapper.vm.openEditor(1)
        await nextTick()

        expect(wrapper.html()).toContain('<td><span>Hello World One</span>')
        expect(wrapper.html()).not.toContain('<td><span>Hello World Two</span>')
        expect(wrapper.html()).toContain('<div class="input-group"><input class="form-control"><button class="btn btn-primary" type="button"> Confirm </button></div>')
        expect(wrapper.html()).toContain('<td><span>Hello World Three</span>')
    })

    test('Hide Editor', async () => {
        const accounts = [
            {
                id: 0,
                name: 'Hello World One',
                secret: 'Testing',
                otp_digits: 8,
                totp_step: 30,
                import: true,
            },
            {
                id: 0,
                name: 'Hello World Two',
                secret: 'Testing',
                otp_digits: 8,
                totp_step: 30,
                import: true,
            },
            {
                id: 0,
                name: 'Hello World Three',
                secret: 'Testing',
                otp_digits: 8,
                totp_step: 30,
                import: true,
            }
        ]

        const wrapper = mount(
            ImportTable,
            {
                props: {
                    accounts: accounts
                },
            }
        );

        wrapper.vm.cloneAccounts()

        wrapper.vm.displayNameEditor = {1: true}
        await nextTick()

        wrapper.vm.closeEditor(1)
        await nextTick()

        expect(wrapper.html()).toContain('<td><span>Hello World One</span>')
        expect(wrapper.html()).toContain('<td><span>Hello World Two</span>')
        expect(wrapper.html()).not.toContain('<div class="input-group"><input class="form-control"><button class="btn btn-primary" type="button"> Confirm </button></div>')
        expect(wrapper.html()).toContain('<td><span>Hello World Three</span>')
    })
})