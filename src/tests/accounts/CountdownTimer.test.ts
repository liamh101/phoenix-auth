import {describe, expect, test} from "vitest";
import {shallowMount} from "@vue/test-utils";
import CountdownTimer from "../../components/accounts/CountdownTimer.vue";

describe('Countdown Timer', async () => {

    test('Custom timer value', async () => {
        const wrapper = shallowMount(
            CountdownTimer,
            {
                props: {
                    timeout: 30,
                }
            }
        )

        expect(wrapper.vm.animationTimeout).toBe('30s')
    })

})