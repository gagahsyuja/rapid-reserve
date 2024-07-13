<script lang="ts">
    import Title from '$lib/component/Title.svelte';
    import Home from '$lib/component/Home.svelte';
    import Popup from '$lib/component/PopupModal.svelte';
    import Button from '$lib/component/Button.svelte';
    import Booking from '$lib/component/page/Booking.svelte';
    import CheckIn from '$lib/component/page/CheckIn.svelte';
    import CheckOut from '$lib/component/page/CheckOut.svelte';
    import { invoke } from '@tauri-apps/api/tauri';

    let showPopup: boolean = false;
    let showBooking: boolean = false;
    let showCheckIn: boolean = false;
    let showCheckOut: boolean = false;

    const checkRoomAvailability = async () => {

        let rooms: string = await invoke('get_all_rooms');
        let json: Array<any> = JSON.parse(rooms);
        json = json.filter(room => !room.occupied);

        if (json.length) {
            return true;
        } else {
            return false;
        }
    }
</script>

<div>
    <Home />
    <Title title="Rapid Reserve" />
    <div class="p-4">
        <button
            class="text-everforest-black text-xl bg-everforest-red p-4 rounded-xl"
            on:click={() => checkRoomAvailability().then(available => available ? showBooking = true : showPopup = true)}
        >
            Booking
        </button>
        <button
            class="text-everforest-black text-xl bg-everforest-red p-4 rounded-xl"
            on:click={() => showCheckIn = true}
        >
            Check-In
        </button>
        <button
            class="text-everforest-black text-xl bg-everforest-red p-4 rounded-xl"
            on:click={() => showCheckOut = true}
        >
            Check-Out
        </button>
    </div>

    {#if showPopup}
        <Popup>
            <div class="flex flex-col">
                <span>No room available</span>
                <Button on:click={() => showPopup = false} name="Close" fg="everforest-black" bg="everforest-green" />
            </div>
        </Popup>
    {/if}

    {#if showBooking}
        <Booking bind:showBooking />
    {/if}

    {#if showCheckIn}
        <CheckIn bind:showCheckIn />
    {/if}

    {#if showCheckOut}
        <CheckOut bind:showCheckOut />
    {/if}
</div>
