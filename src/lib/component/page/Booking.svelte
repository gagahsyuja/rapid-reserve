<script lang="ts">
    import Title from '$lib/component/Title.svelte';
    import Button from '$lib/component/Button.svelte';
    import Modal from '$lib/component/PopupModal.svelte';
    import Confirm from '../ConfirmationModal.svelte';
    import CloseButton from '$lib/component/CloseButton.svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';
    import Invoice from '../Invoice.svelte';

    export let showBooking: boolean = false;

    let fullname: string = '';
    let nik: string = '';
    let checkInDate: string = '';
    let duration: number = 1;
    let contactInfo: string = '';
    let roomId: string = '';
    let addOn: Array<string> = [];
    let roomsAvailable: Array<any> = [];
    let roomPrice: number = 0;

    let showDetail: boolean = false;
    let guestId: number = 0;

    let currentDate = new Date().toJSON().slice(0, 10);

    const resetVariable = () => {
        fullname = '';
        nik = '';
        checkInDate = '';
        duration = 1;
        contactInfo = '';
        roomId = '';
        addOn = [];
        roomPrice = 0;
    }

    $: if (!showDetail) { resetVariable() };

    const getAllRooms = async () => {

        let rooms: string = await invoke('get_all_rooms');

        let arr: Array<any> = JSON.parse(rooms);

        roomsAvailable = arr.filter(room => !room.occupied);
    }

    const getCheckoutDate = (checkinDate: string, duration: number) => {

        let options: object = {
            year: 'numeric',
            month: '2-digit',
            day: '2-digit'
        };

        let date: any = new Date(checkinDate);
        date.setDate(date.getDate() + duration);
        date = date.toLocaleDateString('en-CA', options).toString();
        date = date.replaceAll('/', '-');

        return date;
    }

    $: if (roomId) {
            getRoomPrice(parseInt(roomId))
        }

    const getRoomPrice = async (id: number) => {

        let price: number = await invoke('get_room_price', { roomId: id });

        roomPrice = price;

        return price;
    }

    const getTotalAmount = (): number => {

        let amount: number = 0;

        for (let i of addOn.keys())
        {
            if (addOn[i] === "breakfast")
            {
                amount += 300000;
            }

            else if (addOn[i] === "spa")
            {
                amount += 600000;
            }

            else if (addOn[i] === "gym")
            {
                amount += 200000;
            }
        }

        amount += (roomPrice * duration);

        return amount;
    }

    const addInvoice = async (id: number): Promise<number> => {

        let arrPrice: Array<number> = [];
        let arrAmount: Array<number> = [];

        let obj = {
            items: addOn,
            price: [0],
            amount: [0]
        };

        for (let i of addOn.keys()) {
            if (addOn[i] === "breakfast") {
                arrPrice.push(300000);
                arrAmount.push(1);
            } else if (addOn[i] === "spa") {
                arrPrice.push(600000);
                arrAmount.push(1);
            } else if (addOn[i] === "gym") {
                arrPrice.push(200000);
                arrAmount.push(1);
            }
        }

        obj.price = arrPrice;
        obj.amount = arrAmount;

        let lastId: number = await invoke('add_invoice', {
            guestId: id,
            itemsJson: JSON.stringify(obj),
            amountToPay: getTotalAmount(),
            date: currentDate,
            dueDate: checkInDate,
            hasPaid: false
        });

        return lastId;
    }

    const addReport = async (guestId: number, invoiceId: number): Promise<number> => {

        let id: number = await invoke('add_report', {
            guestId,
            invoiceId,
            roomId: parseInt(roomId),
            checkInDate,
            actualCheckInDate: null,
            checkOutDate: getCheckoutDate(checkInDate, duration),
            actualCheckOutDate: null
        });

        return id;
    }

    const addGuest = async (): Promise<number> => {

        let id: number = await invoke('add_guest', {
            nik,
            roomId: parseInt(roomId),
            fullName: fullname,
            checkInDate,
            checkOutDate: getCheckoutDate(checkInDate, duration),
            duration,
            contactInfo,
            paymentStatus: false,
            isCheckedOut: false
        });

        return id;
    }

    const book = async () => {

        addGuest().then(lastGuestId => {
            guestId = lastGuestId;
            invoke('set_room_occupied', {
                roomId: parseInt(roomId),
                occupied: true
            }).then(() => {
                addInvoice(guestId).then(lastInvoiceId => {
                    addReport(guestId, lastInvoiceId).then(() => {
                        showDetail = true;
                    })
                })
            })
        }).catch(() => {
            resetVariable();
        })
    }

    const getRoom = () => {
        
        getAllRooms().then(() => {
            roomId = roomsAvailable.length ? roomsAvailable[0].number : '';
            getRoomPrice(parseInt(roomId));
        });
    }

    const getRoomBedType = async (id: number) => {

        let type: string = await invoke('get_room_bed_type', { roomId: id });

        return type;
    }

    const getInvoiceDetail = async (guestId: number) => {

        let invoice: string = await invoke('get_invoice_information', { guestId });

        return JSON.parse(invoice);
    }

    onMount(() => {
        getRoom();
    })
</script>

<div>
    <Title title="Book a Room" />
    <div class="mx-[200px]">
        <Modal title="Booking">
            <CloseButton on:click={() => showBooking = false} />
            <form method="POST" on:submit|preventDefault={book} class="flex flex-col p-2">
                <span>Full name</span>
                <input
                    class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2 focus:outline-none"
                    type="text"
                    bind:value={fullname}
                    placeholder="E.g. Gagah Syuja"
                    required
                />
                <span class="pt-3">NIK</span>
                <input
                    class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2 focus:outline-none"
                    type="text"
                    bind:value={nik}
                    placeholder="NIK"
                    required
                />
                <span class="pt-3">Select room</span>
                {#await getAllRooms() then}
                    <select
                        bind:value={roomId}
                        class="m-2 p-2 bg-everforest-black text-everforest-green border-4 border-everforest-green flex flex-row rounded-md color-everforest-green outline-none appearance-none focus:bg-everforet-black focus:outline-none"
                        required
                    >
                        {#if roomsAvailable}
                            {#each roomsAvailable as room}
                                {@const roomId = room.id}
                                {@const roomNumber = room.number}
                                {@const roomBedType = room.bedType}
                                <option class="p-4" value={roomId}>{roomNumber} - {roomBedType}</option>
                            {/each}
                        {:else}
                            <option disabled class="p-4" value={null}>No rooms available</option>
                        {/if}
                    </select>
                {/await}
                <span class="pt-3">Add-On</span>
                <div class="flex flex-row items-center justify-center align-center m-2 pt-2">
                    <label class="container">Breakfast
                        <input type="checkbox" bind:group={addOn} value="breakfast" />
                        <span class="checkmark rounded-md bg-everforest-black border-2 border-everforest-green"></span>
                    </label>
                    <label class="container">Spa
                        <input type="checkbox" bind:group={addOn} value="spa" />
                        <span class="checkmark rounded-md bg-everforest-black border-2 border-everforest-green"></span>
                    </label>
                    <label class="container">Gym
                        <input type="checkbox" bind:group={addOn} value="gym" />
                        <span class="checkmark rounded-md bg-everforest-black border-2 border-everforest-green"></span>
                    </label>
                </div>
                <span class="pt-3">Check in Date</span>
                <input
                    class="m-2 bg-[#3A464C] text-white border-4 border-everforest-green rounded-md p-2"
                    type="date"
                    bind:value={checkInDate}
                    required
                />
                <span class="pt-3">Duration</span>
                <input
                    class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2 focus:outline-none"
                    type="number"
                    min="1"
                    bind:value={duration}
                    placeholder="Room number"
                    required
                />
                <span class="pt-3">Contact info</span>
                <input
                    class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2 focus:outline-none"
                    type="text"
                    bind:value={contactInfo}
                    placeholder="E.g. xxxx-xxxx-xxxx"
                    required
                />
                <br />
                <Button name="Book" fg="everforest-black" bg="everforest-green" />
            </form>
        </Modal>
    </div>
    {#if showDetail}
        {#await getInvoiceDetail(guestId) then invoiceDetail}
            {@const items = JSON.parse(invoiceDetail[0].itemsJson)}
            {#await getRoomBedType(parseInt(roomId)) then bedType}
                <Invoice
                    id={invoiceDetail[0].id}
                    date={invoiceDetail[0].date}
                    dueDate={invoiceDetail[0].dueDate}
                    fullname={fullname}
                    contact={contactInfo}
                    items={items}
                    duration={duration}
                    roomPrice={roomPrice}
                    bedType={bedType}
                    totalAmount={getTotalAmount()}
                    bind:showDetail
                />
            {/await}
        {/await}
    {/if}
</div>

<style>

.container {
  display: block;
  position: relative;
  padding-left: 35px;
  margin: auto;
  cursor: pointer;
  font-size: 18px;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

.container input {
  position: absolute;
  opacity: 0;
  cursor: pointer;
  height: 0;
  width: 0;
}

.checkmark {
  position: absolute;
  top: 0;
  left: 0;
  height: 25px;
  width: 25px;
}

.container:hover input ~ .checkmark {
  background-color: #A7C080;
  transition: 0.2s;
}

.container input:checked ~ .checkmark {
  background-color: #A7C080;
}

.checkmark:after {
  content: "";
  position: absolute;
  display: none;
}

.container input:checked ~ .checkmark:after {
  display: block;
}

.container .checkmark:after {
  left: 9px;
  top: 5px;
  width: 5px;
  height: 10px;
  border: solid white;
  border-width: 0 3px 3px 0;
  -webkit-transform: rotate(45deg);
  -ms-transform: rotate(45deg);
  transform: rotate(45deg);
}

</style>
