<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import Modal from '$lib/component/PopupModal.svelte';
    import Confirm from '$lib/component/ConfirmationModal.svelte';
    import Home from '$lib/component/Home.svelte';
    import Title from '$lib/component/Title.svelte';
    import Header from '$lib/component/Header.svelte';
    import CloseButton from '$lib/component/CloseButton.svelte';
    import Form from '$lib/component/guest/Form.svelte';
    import Legend from '$lib/component/Legend.svelte';
    import { onMount } from 'svelte';

    let fullName: string;
    let nik: string;
    let roomNumber: string;
    let checkInDate: string;
    let checkOutDate: string;
    let contactInfo: string;
    let paymentStatus: boolean = false;
    let showAddButton: boolean = false;
    let guestsList: Array<any> = [];
    let showRemoveConfirmation: boolean = false;
    let idToRemove: any = null;
    let fullnameToRemove: any = null;
    let showEditModal: boolean = false;
    let idToEdit: any = null;
    let searchKeyword: string = '';
    let filteredGuestsList: Array<any> = [];
    let isCheckedOut: boolean = false;

    $: searchKeyword, filteredGuestsList = guestsList.filter(guest => guest.fullName.toLowerCase().includes(searchKeyword.toLowerCase()) || guest.nik.includes(searchKeyword.toLowerCase()));

    const getAllGuests = async () => {

        let guests: string = await invoke('get_all_guests');

        let arr: Array<any> = JSON.parse(guests);

        guestsList = filteredGuestsList = arr.filter(guest => !guest.isCheckedOut);
    }

    const resetVariable = () => {
        
        fullName = '';
        nik = '';
        roomNumber = '';
        checkInDate = '';
        checkOutDate = '';
        contactInfo = '';
        paymentStatus = false;
        isCheckedOut = false;
    }

    const addGuest = async () => {
        invoke("add_guest", {
            nik,
            roomId: parseInt(roomNumber),
            fullName,
            checkInDate,
            checkOutDate,
            contactInfo,
            paymentStatus,
            isCheckedOut
        }).then(() => {
            showAddButton = false;
            resetVariable();
            getAllGuests();
        });

    }

    const removeGuest = async (id: number, fullname: String) => {
        idToRemove = id;
        fullnameToRemove = fullname;
        showRemoveConfirmation = true;
    }

    const confirmRemoveGuest = async () => {
        invoke('remove_guest', { guestId: idToRemove }).then(() => {
            idToRemove = null;
            fullnameToRemove = null;
            showRemoveConfirmation = false;
            getAllGuests();
        });
    }

    const cancelRemoveGuest = async () => {
        idToRemove = null;
        fullnameToRemove = null;
        showRemoveConfirmation = false;
    }

    const getGuestInformation = async (id: number) => {

        let guest: string = await invoke('get_guest_information', { guestId: id });

        return JSON.parse(guest);
    }

    const editGuest = async () => {

        invoke('edit_guest', {
            id: idToEdit,
            nik,
            roomId: parseInt(roomNumber),
            fullName,
            checkInDate: checkInDate,
            checkOutDate: checkOutDate,
            contactInfo,
            paymentStatus
        }).then(() => {
            showEditModal = false
            idToEdit = null;
            resetVariable();
            getAllGuests();
        });
    }

    const showEdit = async (id: number) => {

        let guest = await getGuestInformation(id);

        idToEdit = id;

        fullName = guest[0].fullName;
        nik = guest[0].nik;
        roomNumber = guest[0].roomId.toString();
        checkInDate = guest[0].checkInDate;
        checkOutDate = guest[0].checkOutDate;
        contactInfo = guest[0].contactInfo;
        paymentStatus = guest[0].paymentStatus;

        showEditModal = true;
    }

    onMount(async () => {
        await getAllGuests();
    })
</script>

<div>
    <Legend greenIdentifier="Has Paid" redIdentifier="Has Not Paid" />
    <Home />
    <Title title="Guest" />
    <Header bind:showAddButton bind:searchKeyword whatToAdd="Guest" />

    {#if showAddButton}
        <Modal title="New Guest">
            <CloseButton on:click={() => showAddButton = false} />
            <Form
                on:submit={addGuest}
                bind:fullName
                bind:nik
                bind:roomNumber
                bind:checkInDate
                bind:checkOutDate
                bind:contactInfo
                bind:paymentStatus
                buttonName="Add Guest"
            />
        </Modal>
    {/if}

    <div class="grid grid-cols-4 gap-4 p-8 pb-[7%]">
        {#if guestsList.length}
            {#each filteredGuestsList as guest}
                {@const paymentColorBg = guest.paymentStatus ? "everforest-green" : "everforest-red"}
                <div class="flex flex-col items-center w-full text-everforest-black rounded-xl bg-{paymentColorBg} p-4">
                    <div class="flex flex-col items-center">
                        <span>Room {guest.roomId}</span>
                        <span class="text-xl font-bold">{guest.fullName}</span>
                    </div>
                    <span>{guest.contactInfo}</span>
                    <div class="flex flex-row">
                        <div class="flex flex-col items-center p-4">
                            <span class="font-bold">Check In Date</span>
                            <span>{guest.checkInDate}</span>
                        </div>
                        <div class="flex flex-col items-center p-4">
                            <span class="font-bold">Check Out Date</span>
                            <span>{guest.checkOutDate}</span>
                        </div>
                    </div>
                    <div class="flex flex-row justify-around w-full px-8">
                        <button on:click={() => showEdit(guest.id)} class="border-2 border-everforest-black rounded-md w-[80px] h-[30px]">
                            Edit
                        </button>
                        <button
                            on:click={() => removeGuest(guest.id, guest.fullName)}
                            class="border-2 border-everforest-black rounded-md w-[80px] h-[30px]"
                        >
                            Remove
                        </button>
                    </div>
                </div>
            {/each}
        {:else}
            <h1 class="text-left font-bold text-lg">Guest is empty</h1>
        {/if}
    </div>

    {#if showRemoveConfirmation}
        <Confirm message="You're about the remove guest {fullnameToRemove}">
            <div class="flex flex-row items-center justify-evenly">
                <button
                    class="bg-everforest-red text-everforest-black px-6 py-2 rounded-md"
                    on:click={confirmRemoveGuest}
                >Remove
                </button>
                <button
                    class="bg-everforest-green text-everforest-black px-6 py-2 rounded-md"
                    on:click={cancelRemoveGuest}
                >Cancel
                </button>
            </div>
        </Confirm>
    {/if}

    {#if showEditModal}
        <Modal title="Edit Guest">
            <CloseButton on:click={() => {showEditModal = false; idToEdit = null; resetVariable()}} />
            <Form
                on:submit={editGuest}
                bind:fullName
                bind:nik
                bind:roomNumber
                bind:checkInDate
                bind:checkOutDate
                bind:contactInfo
                bind:paymentStatus
                buttonName="Edit Guest"
            />
        </Modal>
    {/if}
</div>
