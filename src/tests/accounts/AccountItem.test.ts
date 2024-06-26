import {describe, expect, test} from "vitest";
import {shallowMount} from "@vue/test-utils";
import AccountItem from "../../components/accounts/AccountItem.vue";

describe('Account Item', async () => {
    test('Display Token Item', async () => {
        const wrapper = shallowMount(
            AccountItem,
            {
                props: {
                    accountId: 1,
                    accountName: 'Main Account'
                }
            }
        );

        expect(wrapper.html()).toContain('Main Account')
        expect(wrapper.html()).not.toContain('delete-account')

    })

    test('Display Manage Item', async () => {
        const wrapper = shallowMount(
            AccountItem,
            {
                props: {
                    accountId: 1,
                    accountName: 'Main Account',
                    manage: true,
                }
            }
        );

        expect(wrapper.html()).toContain('Main Account')
        expect(wrapper.html()).toContain('delete-account')
    })
})