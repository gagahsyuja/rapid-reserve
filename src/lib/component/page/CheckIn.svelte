<script lang="ts">
    import Title from '$lib/component/Title.svelte';
    import Button from '$lib/component/Button.svelte';
    import Popup from '$lib/component/PopupModal.svelte';
    import Modal from '$lib/component/PopupModal.svelte';
    import CloseButton from '$lib/component/CloseButton.svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';

    export let showCheckIn: boolean = false;

    let guests: Array<any> = [];
    let filteredGuests: Array<any> = [];

    let searchKeyword: string = '';

    let showPopup: boolean = false;

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
                    .then(async () => {
                        showPopup = true;
                        await getAllGuests();
                });
        });
    }

    onMount(() => {
        getAllGuests().then(() => console.log(guests));
    })
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
                        class="flex flex-row items-center justify-center w-full text-everforest-black rounded-xl bg-everforest-green p-8 hover:bg-everforest-red h-[100px] mb-4"
                        on:click={() => checkin(guest.id)}
                    >
                        <div class="flex flex-col items-center">
                            <span>Room {guest.roomId}</span>
                            <span class="text-xl">{guest.fullName}</span>
                        </div>
                        <div class="flex flex-row">
                            <div class="flex flex-col items-center p-4">
                                <span>Check In Date</span>
                                <span>{guest.checkInDate}</span>
                            </div>
                            <div class="flex flex-col items-center p-4">
                                <span>Check Out Date</span>
                                <span>{guest.checkOutDate}</span>
                            </div>
                        </div>
                    </button>
                {/each}
            {:else}
                <span>Guest is empty</span>
            {/if}
        </div>
    </div>
    </Modal>

    {#if showPopup}
        <Popup>
            <div class="flex flex-col">
                <span>Check in successful</span>
                <Button on:click={() => showPopup = false} name="Thank you" fg="everforest-black" bg="everforest-green" />
            </div>
        </Popup>
    {/if}
{/await}
