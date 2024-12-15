import {afterEach, describe, expect, test} from "vitest";
import {clearMocks, mockIPC} from "@tauri-apps/api/mocks";
import SettingsPage from "../../components/settings/SettingsPage.vue";
import {flushPromises, shallowMount} from "@vue/test-utils";

describe('Settings Page', async () => {
    afterEach(() => {
        clearMocks()
    })

    test('Emit Sync Attempt', async () => {
        let syncMade = false;

        mockIPC((cmd) => {
            if (cmd !== 'attempt_sync_with_remote') {
                return 'INVALID'
            }

            syncMade = true;
        })

        const wrapper = shallowMount(SettingsPage);

        wrapper.vm.syncRequired = true;

        wrapper.vm.performSyncIfRequired();

        await flushPromises();

        expect(syncMade).toBeTruthy();
    })

    test('Emit Sync Attempt - Invalid', async () => {
        let syncMade = false;

        mockIPC((cmd) => {
            if (cmd !== 'attempt_sync_with_remote') {
                return 'INVALID'
            }

            syncMade = true;
        })

        const wrapper = shallowMount(SettingsPage);

        wrapper.vm.syncRequired = false;

        wrapper.vm.performSyncIfRequired();

        await flushPromises();

        expect(syncMade).toBeFalsy();
    })

    test('Sync When Going Back to Settings List', async () => {
        let syncMade = false;

        mockIPC((cmd) => {
            if (cmd !== 'attempt_sync_with_remote') {
                return 'INVALID'
            }

            syncMade = true;
        })

        const wrapper = shallowMount(SettingsPage);

        wrapper.vm.syncRequired = true;

        wrapper.vm.reset();

        await flushPromises();

        expect(syncMade).toBeTruthy();
    })

    test('Sync When Going to Token List', async () => {
        let syncMade = false;

        mockIPC((cmd) => {
            if (cmd !== 'attempt_sync_with_remote') {
                return 'INVALID'
            }

            syncMade = true;
        })

        const wrapper = shallowMount(SettingsPage);

        wrapper.vm.syncRequired = true;

        wrapper.vm.showTokens();

        await flushPromises();

        expect(syncMade).toBeTruthy();
    })

})