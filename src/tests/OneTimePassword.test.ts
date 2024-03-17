import {expect, test, afterEach, beforeEach} from 'vitest'
import { mockIPC, clearMocks } from "@tauri-apps/api/mocks"
import { flushPromises, mount} from "@vue/test-utils";

import OneTimePassword from "../components/accounts/OneTimePassword.vue";

beforeEach(() => {
    mockIPC((cmd, args) => {
        if (cmd !== "get_one_time_password_for_account") {
            return 'INVALID'
        }

        if (args.account === 12) {
            return "456908";
        }

        return "";
    })
})

afterEach(() => {
    clearMocks()
})

test('Fetch Valid 2FA', async () => {
    const wrapper = mount(
        OneTimePassword,
        {
            propsData: {
                accountId: 12
            }
        }
    )
    wrapper.vm.getOneTimePassword();

    await flushPromises();

    expect(wrapper.vm.otp).toBe("456908")
    expect(wrapper.html()).toBe("<button>456908</button>")
})

test('Invalid Account', async () => {
    const wrapper = mount(
        OneTimePassword,
        {
            propsData: {
                accountId: 1
            }
        }
    )
    wrapper.vm.getOneTimePassword();

    await flushPromises();

    expect(wrapper.vm.otp).toBe("")
    expect(wrapper.html()).toBe("<button></button>")
})

test('On Hover Exit', async () => {
    const wrapper = mount(
        OneTimePassword,
        {
            propsData: {
                accountId: 1
            }
        }
    )
    wrapper.vm.otp = 'Test';
    wrapper.vm.onExit()

    expect(wrapper.vm.otp).toBe('------')
})