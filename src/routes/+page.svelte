<script lang="ts">
    import Title from '$lib/component/Title.svelte';
    import Success from '$lib/component/Success.svelte';
    import Booking from '$lib/component/page/Booking.svelte';
    import CheckIn from '$lib/component/page/CheckIn.svelte';
    import CheckOut from '$lib/component/page/CheckOut.svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { pushState } from '$app/navigation';

    let showPopup: boolean = false;
    let showBooking: boolean = false;
    let showCheckIn: boolean = false;
    let showCheckOut: boolean = false;

    let message: string = '';

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

    const checkInAvailability = async () => {

        let guests: string = await invoke('get_all_guests');
        let json: Array<any> = JSON.parse(guests);
        json = json.filter(guest => !guest.paymentStatus);

        if (json.length) {
            return true;
        } else {
            return false;
        }
    }

    const checkOutAvailability = async () => {

        let guests: string = await invoke('get_all_guests');
        let json: Array<any> = JSON.parse(guests);
        json = json.filter(guest => guest.paymentStatus && !guest.isCheckedOut);

        if (json.length) {
            return true;
        } else {
            return false;
        }
    }
</script>

{#await invoke('get_database_path') then path}
    <Title title="Rapid Reserve" />
    <h1 class="text-center font-bold text-everforest-white text-xl mt-[2%]">
        Database path: <span class="font-medium">{path}</span>
    </h1>
    <div class="grid grid-rows-4 grid-flow-col gap-4 h-[500px] text-4xl font-bold px-[10%] text-everforest-black mt-[1%]">
        <a href="/management/room" class="bg-everforest-orange row-span-2 text-center rounded-xl pt-[20%] hover:bg-everforest-orange/80">Manage Room</a>
        <a href="/management/guest" class="bg-everforest-orange row-span-2 text-center rounded-xl pt-[20%] hover:bg-everforest-orange/80">Manage Guest</a>
        <button
            class="bg-everforest-aqua row-span-2 text-center rounded-xl hover:bg-everforest-aqua/80"
            on:click={() => checkRoomAvailability().then(available => {
                available ? showBooking = true : showPopup = true;
                message = 'No room available'
            })}
        >Booking</button>
        <button
            class="bg-everforest-aqua text-center rounded-xl hover:bg-everforest-aqua/80"
            on:click={() => checkInAvailability().then(available => {
                available ? showCheckIn = true : showPopup = true;
                message = 'No guest to be checked in';
            })}
        >Check In</button>
        <button
            class="bg-everforest-aqua text-center rounded-xl hover:bg-everforest-aqua/80"
            on:click={() => checkOutAvailability().then(available => {
                available ? showCheckOut = true : showPopup = true;
                message = 'No guest to be checked out';
            })}
        >Check Out</button>
        <a href="/invoice" class="bg-everforest-purple text-center row-span-2 rounded-xl pt-[22%] hover:bg-everforest-purple/80">View Invoice</a>
        <a href="/report" class="bg-everforest-purple row-span-2 text-center rounded-xl pt-[22%] hover:bg-everforest-purple/80">View Report</a>
    </div>

    {#if showPopup}
    <Success success={false} on:click={() => showPopup = false} message={message} />
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
{/await}
