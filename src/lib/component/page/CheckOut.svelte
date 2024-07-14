<script lang="ts">
    import Button from '$lib/component/Button.svelte';
    import Popup from '$lib/component/PopupModal.svelte';
    import Modal from '$lib/component/PopupModal.svelte';
    import CloseButton from '$lib/component/CloseButton.svelte';
    import Success from '$lib/component/Success.svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';

    export let showCheckOut: boolean = false;

    let guests: Array<any> = [];
    let filteredGuests: Array<any> = [];

    let searchKeyword: string = '';

    let showPopup: boolean = false;

    let currentDate = new Date().toJSON().slice(0, 10);

    $: searchKeyword, filteredGuests = guests.filter(guest => guest.fullName.toLowerCase().includes(searchKeyword.toLowerCase()) || guest.nik.includes(searchKeyword.toLowerCase()));
    
    const getAllGuests = async () => {

        let json: string = await invoke('get_all_guests');

        let arr: Array<any> = JSON.parse(json);

        guests = filteredGuests = arr.filter(guest => guest.paymentStatus && !guest.isCheckedOut);
    }

    const getGuestInformation = async (id: number) => {

        let guest: string = await invoke('get_guest_information', { guestId: id });

        return JSON.parse(guest);
    }

    const checkOutGuest = async (id: number, status: boolean) => {
        await invoke('set_check_out_status', { guestId: id, status });
    }

    const checkout = async (id: number) => {

        getGuestInformation(id).then(guest => {
            invoke('set_room_occupied', { roomId: guest[0].roomId, occupied: false }).then(() => {
                checkOutGuest(id, true).then(() => {
                    invoke('set_report_actual_check_out_date', { guestId: id, date: currentDate }).then(() => {
                        getAllGuests();
                        showPopup = true;
                    })
                })
            });
        });
    }

    onMount(() => {
        getAllGuests().then(() => console.log(guests));
    })
</script>

{#await getAllGuests() then}
    <Modal title="Check Out">
        <CloseButton on:click={() => showCheckOut = false} />

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
                            class="flex flex-row items-center justify-evenly w-full text-everforest-black rounded-xl bg-everforest-green hover:bg-everforest-red h-[100px] mb-4"
                            on:click={() => checkout(guest.id)}
                        >
                            <div class="flex flex-col items-center">
                                <span>Room {guest.roomId}</span>
                                <span class="text-xl font-bold">{guest.fullName}</span>
                            </div>
                            <div class="flex flex-row">
                                <div class="flex flex-col items-center p-4">
                                    <span>Due Date</span>
                                    <span class="font-bold">{guest.checkOutDate}</span>
                                </div>
                            </div>
                        </button>
                    {/each}
                {:else}
                    <h1 class="text-center font-bold text-xl">No guest to be checked out</h1>
                {/if}
            </div>
        </div>
    </Modal>

    {#if showPopup}
        <Success message="Check out successful" on:click={() => showPopup = false} />
    {/if}
{/await}
