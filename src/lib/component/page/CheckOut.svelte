<script lang="ts">
    import Button from '$lib/component/Button.svelte';
    import Modal from '$lib/component/PopupModal.svelte';
    import CloseButton from '$lib/component/CloseButton.svelte';
    import Success from '$lib/component/Success.svelte';
    import Invoice from '../Invoice.svelte';
    import { invoke } from '@tauri-apps/api/tauri';

    export let showCheckOut: boolean = false;

    let guests: Array<any> = [];
    let filteredGuests: Array<any> = [];

    let searchKeyword: string = '';

    let showDetail: boolean = false;
    let showAdditional: boolean = false;

    let mineralWaterAmount: number = 0;
    let snacksAmount: number = 0;

    let guestId: number = 0;
    let amountToPay: number = 0;
    let json: any = null;

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

    const getRoomInformation = async (id: number) => {
        
        let room: string = await invoke('get_room_information', { id });

        return JSON.parse(room);
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
                        guestId = id;
                        showAdditional = true;
                    })
                })
            });
        });
    }

    const getInvoiceInformation = async (id: number) => {

        let invoice: string = await invoke('get_invoice_information', { guestId: id });

        let json = JSON.parse(invoice);

        json[0].itemsJson = JSON.parse(json[0].itemsJson);

        return json;
    }

    const addAdditional = (id: number) => {

        getGuestInformation(id).then(guest => {

            getRoomInformation(guest[0].roomId).then(room => {

                amountToPay = (room[0].price * guest[0].duration);
                
                getInvoiceInformation(id).then(information => {
                    
                    json = information[0].itemsJson;

                    for (let i of json.price.keys()) {
                        amountToPay += json.price[i];
                    }

                    if (mineralWaterAmount !== 0) {
                        json.items.push("mineral water");
                        json.amount.push(mineralWaterAmount);
                        json.price.push(15000);
                        amountToPay += (15000 * mineralWaterAmount);
                    }

                    if (snacksAmount !== 0) {
                        json.items.push("snacks")
                        json.amount.push(snacksAmount);
                        json.price.push(30000);
                        amountToPay += (30000 * snacksAmount);
                    }

                    invoke('set_additional_items', { guestId: id, itemsJson: JSON.stringify(json), amountToPay })
                        .then(() => showDetail = true);
                });
            })
        })
    }
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

    <!-- {#if showPopup} -->
    <!--     <Success message="Check out successful" on:click={() => showPopup = false} /> -->
    <!-- {/if} -->

    {#if showDetail}
        {#await getInvoiceInformation(guestId) then invoiceDetail}
            {#await getGuestInformation(guestId) then guest}
                {#await getRoomInformation(guest[0].roomId) then room}
                    <Invoice
                        id={invoiceDetail[0].id}
                        date={invoiceDetail[0].date}
                        dueDate={invoiceDetail[0].dueDate}
                        fullname={guest[0].fullName}
                        contact={guest[0].contactInfo}
                        items={json}
                        duration={guest[0].duration}
                        roomPrice={room[0].price}
                        bedType={room[0].bedType}
                        totalAmount={amountToPay}
                        bind:showDetail
                    />
                {/await}
            {/await}
        {/await}
    {/if}

    {#if showAdditional}
        <Modal title="Additional Items">
            <div class="flex flex-col justify-center pt-8">
                <div class="flex flex-row items-center font-bold justify-center">
                    <h1 class="min-w-[180px]">MINERAL WATER</h1>
                    <input
                        class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2 focus:outline-none min-w-[260px] max-w-[260px]"
                        type="text"
                        inputmode="numeric"
                        maxlength="5"
                        bind:value={mineralWaterAmount}
                        placeholder="Amount of mineral water"
                        required
                    />
                </div>
                <div class="flex flex-row items-center font-bold justify-center mb-[10%]">
                    <h1 class="min-w-[180px]">SNACKS</h1>
                    <input
                        class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2 focus:outline-none min-w-[260px] max-w-[260px]"
                        type="text"
                        inputmode="numeric"
                        maxlength="5"
                        bind:value={snacksAmount}
                        placeholder="Amount of snacks"
                        required
                    />
                </div>
                <Button on:click={() => { addAdditional(guestId); showAdditional = false }} name="Continue" fg="everforest-black" bg="everforest-green" />
            </div>
        </Modal>
    {/if}
{/await}
