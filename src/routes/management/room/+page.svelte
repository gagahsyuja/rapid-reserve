<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import Home from '$lib/component/Home.svelte';
    import Title from '$lib/component/Title.svelte';
    import Header from '$lib/component/Header.svelte';
    import Modal from '$lib/component/PopupModal.svelte';
    import CloseButton from '$lib/component/CloseButton.svelte';
    import Form from '$lib/component/room/Form.svelte';
    import Confirm from '$lib/component/ConfirmationModal.svelte';
    import Legend from '$lib/component/Legend.svelte';
    import { onMount } from 'svelte';

    let showAddButton: boolean = false;
    let searchKeyword: string = '';

    let roomNumber: string;
    let bedType: string;
    let occupied: boolean = false;
    let price: string;

    let roomsList: Array<any> = [];
    let filteredRoomsList: Array<any> = [];

    let idToRemove: any = null;
    let numberToRemove: any = null;
    let showRemoveConfirmation: boolean = false;

    let idToEdit: any = null;
    let showEditModal: boolean = false;

    $: searchKeyword, filteredRoomsList = roomsList.filter(room => room.number.toString().includes(searchKeyword) || room.bedType.includes(searchKeyword))

    const resetVariable = () => {
        roomNumber = '';
        bedType = '';
        occupied = false;
        price = '';
    }

    const addRoom = async () => {
        invoke('add_room', {
            number: parseInt(roomNumber),
            bedType,
            occupied,
            price: parseInt(price)
        }).then(() => {
            showAddButton = false;
            resetVariable();
            getAllRooms();
        })
    }

    const getAllRooms = async () => {

        let rooms: string = await invoke('get_all_rooms');

        roomsList = filteredRoomsList = JSON.parse(rooms);
    }

    const confirmRemoveRoom = async () => {

        invoke('remove_room', { roomId: idToRemove }).then(() => {
            idToRemove = null;
            numberToRemove = null;
            showRemoveConfirmation = false;
            
            getAllRooms();
        });
    }

    const cancelRemoveRoom = () => { idToRemove = null; numberToRemove = null;
        showRemoveConfirmation = false;
    }

    const removeRoom = (roomId: number, roomNumber: number) => {
        idToRemove = roomId;
        numberToRemove = roomNumber;
        showRemoveConfirmation = true;
    }

    const getRoomInformation = async (roomId: number)  => {

        let room: string = await invoke('get_room_information', { id: roomId });

        return JSON.parse(room);
    }

    const showEdit = async (roomId: number) => {

        let room = await getRoomInformation(roomId);

        idToEdit = roomId;

        roomNumber = room[0].number;
        bedType = room[0].bedType;
        occupied = room[0].occupied;
        price = room[0].price.toString();

        showEditModal = true;
    }

    const editRoom = async () => {
        invoke('edit_room', {
            id: idToEdit,
            number: parseInt(roomNumber),
            bedType,
            occupied,
            price: parseInt(price)
        }).then(() => {
            idToEdit = null;
            showEditModal = false;
            resetVariable();
            getAllRooms();
        });

    }

    onMount(async () => {
        await getAllRooms();
    })
</script>

<div>
    <Legend greenIdentifier="Not Occupied" redIdentifier="Occupied" />
    <Home />
    <Title title="Room" />
    <Header bind:showAddButton bind:searchKeyword whatToAdd="Room" />

    {#if showAddButton}
        <Modal title="New Room">
            <CloseButton on:click={() => showAddButton = false} />
            <Form
                on:submit={addRoom}
                bind:roomNumber
                bind:bedType
                bind:price
                bind:occupied
                buttonName="Add Room"
            />
        </Modal>
    {/if}

    <div class="grid grid-cols-4 gap-4 p-8">
        {#if roomsList.length}
            {#each filteredRoomsList as room}
                {@const bgColor = !room.occupied ? "everforest-green" : "everforest-red"}
                <div class="flex flex-col items-center w-full text-everforest-black rounded-xl bg-{bgColor} p-4">
                    <span class="font-bold">Room {room.number}</span>
                    <span>{room.bedType}</span>
                    <span>IDR {room.price}</span>
                    <div class="flex flex-row justify-around w-full px-8 mt-4">
                        <button on:click={() => showEdit(room.id)} class="border-2 border-everforest-black rounded-md w-[80px] h-[30px]">
                            Edit
                        </button>
                        <button
                            on:click={() => removeRoom(room.id, room.number)}
                            class="border-2 border-everforest-black rounded-md w-[80px] h-[30px]"
                        >
                            Remove
                        </button>
                    </div>
                </div>
            {/each}
        {:else}
            <h1 class="text-left font-bold text-lg">No room listed</h1>
        {/if}
    </div>

    {#if showRemoveConfirmation}
        <Confirm message="You're about the remove room {numberToRemove}">
            <div class="flex flex-row items-center justify-evenly">
                <button
                    class="bg-everforest-red text-everforest-black px-6 py-2 rounded-md"
                    on:click={confirmRemoveRoom}
                >Remove
                </button>
                <button
                    class="bg-everforest-green text-everforest-black px-6 py-2 rounded-md"
                    on:click={cancelRemoveRoom}
                >Cancel
                </button>
            </div>
        </Confirm>
    {/if}

    {#if showEditModal}
        <Modal title="Edit Room">
            <CloseButton on:click={() => {showEditModal = false; idToEdit = null; resetVariable()}}/>
            <Form
                on:submit={editRoom}
                bind:roomNumber
                bind:bedType
                bind:price
                bind:occupied
                buttonName="Edit Room"
            />
        </Modal>
    {/if}
</div>
