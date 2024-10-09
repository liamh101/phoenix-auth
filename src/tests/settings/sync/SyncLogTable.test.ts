import {afterEach, describe, expect, test} from "vitest";
import {flushPromises, shallowMount} from "@vue/test-utils";
import {clearMocks, mockIPC} from "@tauri-apps/api/mocks";
import SyncLogTable from "../../../components/settings/sync/SyncLogTable.vue";
import {SyncLog, SyncLogType} from "../../../composables/Commands";

describe('Sync Log Table', async () => {
    afterEach(() => {
        clearMocks()
    })

    test('Get Logs', async () => {
        mockIPC((cmd) => {
            if (cmd !== 'get_sync_logs') {
                return 'INVALID'
            }

            return '[{"id": 1, "log": "Error One", "timestamp": 1726865841, "log_type": "ERROR"}, {"id": 2, "log": "Error Two", "timestamp": 1695225441, "log_type": "ERROR"}]';
        })

        await flushPromises();

        expect(wrapper.html()).toContain('<td>Error One</td>')
        expect(wrapper.html()).toContain('<td>9/20/2024, 9:57 PM</td>')
        expect(wrapper.html()).toContain('<td>Error Two</td>')
        expect(wrapper.html()).toContain('<td>9/20/2023, 4:57 PM</td>')
        expect(wrapper.html()).not.toContain('<td colspan="2" class="text-center"> Sync Log Empty </td>')
    });

    test('No Logs', async () => {
        mockIPC((cmd) => {
            if (cmd !== 'get_sync_logs') {
                return 'INVALID'
            }

            return '[]';
        })

        const wrapper = shallowMount(SyncLogTable)

        await flushPromises();

        expect(wrapper.html()).toContain('<td colspan="2" class="text-center"> Sync Log Empty </td>')
    });

    test('Date Format', async () => {
        mockIPC((cmd) => {
            if (cmd !== 'get_sync_logs') {
                return 'INVALID'
            }

            return '[]';
        })

        const wrapper = shallowMount(SyncLogTable)

        const syncLog: SyncLog = {
            id: 1,
            log: "Hello world",
            log_type: SyncLogType.ERROR,
            timestamp: 1726865841,
        };

        const result = wrapper.vm.formatTimestamp(syncLog)

        expect(result).toBe('9/20/2024, 8:57 PM')
    });

    test('Log Error Class', async () => {
        mockIPC((cmd) => {
            if (cmd !== 'get_sync_logs') {
                return 'INVALID'
            }

            return '[]';
        })

        const wrapper = shallowMount(SyncLogTable)

        const syncLog: SyncLog = {
            id: 1,
            log: "Hello world",
            log_type: SyncLogType.ERROR,
            timestamp: 1726865841,
        };

        const result = wrapper.vm.getRowColour(syncLog)

        expect(result).toBe('table-danger')
    });
});