import { describe, test, expect } from "vitest";
import {mount} from "@vue/test-utils";

import AccountButton from "../../components/accounts/AccountButton.vue";


describe('Account Button', () => {
    test('Toggle Label Button', async () => {
        const wrapper = mount(
            AccountButton,
            {
                props: {
                    buttonColour: 'ffffff'
                },
            }
        );

        wrapper.vm.toggleButton();
        await wrapper.vm.$nextTick()

        expect(wrapper.vm.showLabelButton).toBeTruthy();
        expect(wrapper.emitted().displayPassword).toBeTruthy();
        expect(wrapper.emitted().displayPassword.length).toBe(1)

        expect(wrapper.emitted().displayLabel).toBeFalsy();
    })

    test('Toggle Password Button', async () => {
        const wrapper = mount(
            AccountButton,
            {
                props: {
                    buttonColour: 'ffffff'
                },
            }
        );

        wrapper.vm.showLabelButton = true;

        wrapper.vm.toggleButton();
        await wrapper.vm.$nextTick()

        expect(wrapper.vm.showLabelButton).toBeFalsy();
        expect(wrapper.emitted().displayLabel).toBeTruthy();
        expect(wrapper.emitted().displayLabel.length).toBe(1)

        expect(wrapper.emitted().displayPassword).toBeFalsy();
    })

    test('Formatted Colour', async () => {
        const wrapper = mount(
            AccountButton,
            {
                props: {
                    buttonColour: 'ffffff'
                },
            }
        );

        expect(wrapper.vm.formattedColour).toBe('#ffffff')
    })
});