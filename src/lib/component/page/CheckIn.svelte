<script lang="ts">
    import Title from '$lib/component/Title.svelte';
    import Button from '$lib/component/Button.svelte';
    import Popup from '$lib/component/PopupModal.svelte';
    import Modal from '$lib/component/PopupModal.svelte';
    import CloseButton from '$lib/component/CloseButton.svelte';
    import Success from '../Success.svelte';
    import { invoke } from '@tauri-apps/api/tauri';

    export let showCheckIn: boolean = false;

    let guests: Array<any> = [];
    let filteredGuests: Array<any> = [];

    let searchKeyword: string = '';

    let showPopup: boolean = false;

    let currentDate = new Date().toJSON().slice(0, 10);

    $: searchKeyword, filteredGuests = guests.filter(guest => guest.fullName.toLowerCase().includes(searchKeyword.toLowerCase()) || guest.nik.includes(searchKeyword.toLowerCase()));
    
    const getAllGuests = async () => {

        let json: string = await invoke('get_all_guests');

        let arr: Array<any> = JSON.parse(json);

        guests = filteredGuests = arr.filter(guest => !guest.paymentStatus);
    }

    const getGuestInformation = async (id: number) => {

        let guest: string = await invoke('get_guest_information', { guestId: id });

        return JSON.parse(guest);
    }

    const checkin = async (id: number) => {

        invoke('set_payment_status', { guestId: id, status: true })
            .then(() => {
                invoke('set_invoice_payment_status', { guestId: id, status: true })
                    .then(() => {
                        invoke('set_report_actual_check_in_date', { guestId: id, date: currentDate })
                            .then(async () => {
                                showPopup = true;
                                await getAllGuests();
                            });
                    });
            });
    }
</script>

{#await getAllGuests() then}
    <Modal title="Check In">
        <CloseButton on:click={() => showCheckIn = false} />

    <div class="flex flex-col justify-center">
        <input
            class="bg-everforest-black border-4 border-everforest-green shadow-md focus:outline-none w-[80%] h-[50px] text-everforest-white placeholder:text-everforest-white p-4 rounded-3xl mt-8 mb-6 self-center"
            type="text"
            placeholder="Search name..."
            bind:value={searchKeyword}
        />

        <div class="h-[700px] overflow-scroll">
            {#if guests.length}
                {#each filteredGuests as guest}
                    <button
                        class="flex flex-row items-center justify-evenly w-full m-auto text-everforest-black rounded-xl bg-everforest-green hover:bg-everforest-red h-[100px] mb-4"
                        on:click={() => checkin(guest.id)}
                    >
                        <div class="flex flex-col items-center">
                            <span class="">Room {guest.roomId}</span>
                            <span class="text-xl font-bold">{guest.fullName}</span>
                        </div>
                        <div class="flex flex-col items-center p-4">
                            <span>Due Date</span>
                            <span class="font-bold">{guest.checkInDate}</span>
                        </div>
                    </button>
                {/each}
            {:else}
                <h1 class="text-center font-bold text-xl">No guest to be checked in</h1>
            {/if}
        </div>
    </div>
    </Modal>

    {#if showPopup}
        <Success message="Check in successful" on:click={() => showPopup = false} />
    {/if}
{/await}
