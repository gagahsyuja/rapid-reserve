<script lang="ts">
    import Button from '$lib/component/Button.svelte';
    import Popup from '$lib/component/PopupModal.svelte';
    import Modal from '$lib/component/PopupModal.svelte';
    import CloseButton from '$lib/component/CloseButton.svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';

    export let showCheckOut: boolean = false;

    let guests: Array<any> = [];
    let filteredGuests: Array<any> = [];

    let searchKeyword: string = '';

    let showPopup: boolean = false;

    $: searchKeyword, filteredGuests = guests.filter(guest => guest.fullName.toLowerCase().includes(searchKeyword.toLowerCase()) || guest.nik.includes(searchKeyword.toLowerCase()));
    
    const getAllGuests = async () => {

        let json: string = await invoke('get_all_guests');

        let arr: Array<any> = JSON.parse(json);

        guests = filteredGuests = arr.filter(guest => guest.paymentStatus);
    }

    const getGuestInformation = async (id: number) => {

        let guest: string = await invoke('get_guest_information', { guestId: id });

        return JSON.parse(guest);
    }

    const removeGuest = async (id: number) => {
        await invoke('remove_guest', { guestId: id });
    }

    const checkout = async (id: number) => {

        getGuestInformation(id).then(guest => {
            invoke('set_room_occupied', { roomId: guest[0].roomId, occupied: false }).then(() => {
                removeGuest(id).then(() => {
                    getAllGuests();
                    showPopup = true;
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
                            class="flex flex-row items-center justify-center w-full text-everforest-black rounded-xl bg-everforest-green hover:bg-everforest-red h-[100px] mb-4"
                            on:click={() => checkout(guest.id)}
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
                <span>Check out successful</span>
                <Button on:click={() => showPopup = false} name="Thank you" fg="everforest-black" bg="everforest-green" />
            </div>
        </Popup>
    {/if}
{/await}
