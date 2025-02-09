import {afterEach, describe, expect, test} from "vitest";
import {clearMocks, mockIPC} from "@tauri-apps/api/mocks";
import AppearanceSettings from "../../components/settings/AppearanceSettings.vue";
import {flushPromises, shallowMount} from "@vue/test-utils";
import {THEME_MODES} from "../../composables/Commands";

describe('Appearance Settings', async () => {
    afterEach(() => {
        clearMocks()
    })

    test('Get Settings', async () => {
        mockIPC((cmd) => {
            if (cmd !== 'get_settings') {
                return 'INVALID'
            }

            return {id: 1, theme: "DARK"}
        })

        const wrapper = shallowMount(AppearanceSettings);
        await flushPromises();

        expect(wrapper.vm.themeMode).toBe(THEME_MODES.DARK);
    })

    test('Save Settings', async () => {
        mockIPC((cmd, args) => {
            if (cmd === 'get_settings') {
                return {id: 1, theme: "DARK"}
            }

            if (cmd === 'save_settings' && args.theme === 2) {
                return {id: 1, theme: "LIGHT"}
            }

            return 'INVALID';
        })

        const wrapper = shallowMount(AppearanceSettings);
        await flushPromises();

        wrapper.vm.themeMode = THEME_MODES.LIGHT;
        wrapper.vm.saveTheme()

        expect(wrapper.emitted().themeChange).toBeTruthy()
        expect(wrapper.emitted().themeChange[0]).toStrictEqual([THEME_MODES.LIGHT])
    })
})