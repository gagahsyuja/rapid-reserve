<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from "svelte";
    import { faPrint } from '@fortawesome/free-solid-svg-icons';
    import Invoice from '$lib/component/Invoice.svelte';
    import Legend from '$lib/component/Legend.svelte';
    import Title from '$lib/component/Title.svelte';
    import Fa from 'svelte-fa';
    import Home from '$lib/component/Home.svelte';

    let invoiceList: Array<any> = [];

    let filteredList: Array<any> = [];

    let showDetail: boolean = false;

    let invoiceId: number = 0;

    const getAllInvoices = async () => {

        let invoices: string = await invoke('get_all_invoices');

        invoiceList = filteredList =  JSON.parse(invoices);
    }

    const getGuestInformation = async (id: number) => {
        
        let guest: string = await invoke('get_guest_information', { guestId: id });

        return JSON.parse(guest);
    }

    const getInvoiceInformation = async (id: number) => {

        let invoice: string = await invoke('get_invoice_information', { id });
        
        return JSON.parse(invoice);
    }

    const getRoomPrice = async (id: number) => {

        let price: number = await invoke('get_room_price', { roomId: id });

        return price;
    }

    const getRoomBedType = async (id: number) => {
        
        let type: string = await invoke('get_room_bed_type', { roomId: id });

        return type;
    }

    const getInvoiceDetail = async (id: number) => {

        let obj = {
            id,
            fullname: '',
            contact: '',
            date: '',
            dueDate: '',
            items: [],
            bedType: '',
            roomPrice: 0,
            duration: 0,
            totalAmount: 0
        };

        let invoice = await getInvoiceInformation(id);
        let guest = await getGuestInformation(invoice[0].guestId);

        obj.fullname = guest[0].fullName;
        obj.contact = guest[0].contactInfo;
        obj.date = invoice[0].date;
        obj.dueDate = invoice[0].dueDate;
        obj.items = JSON.parse(invoice[0].itemsJson);
        obj.bedType = await getRoomBedType(guest[0].roomId)
        obj.roomPrice = await getRoomPrice(guest[0].roomId);
        obj.duration = guest[0].duration;
        obj.totalAmount = invoice[0].amountToPay;

        return obj;
    }

    const viewDetail = (id: number) => {
        showDetail = true;
        invoiceId = id;
    }

    onMount(() => {
        getAllInvoices().then(() => console.log(invoiceList));
    });
</script>


<Home />
<Legend greenIdentifier="Has paid" redIdentifier="Has not paid" />
<Title title="Invoice" />
<div class="w-[50%] m-auto">
    <div class="flex flex-col justify-center">
        <div class="h-[700px] overflow-scroll">
            {#if invoiceList.length}
                {#each invoiceList as invoice}
                    {#await getGuestInformation(invoice.guestId) then guest}
                        {@const id = invoice.id}
                        {@const amount = invoice.amountToPay}
                        {@const guestName = guest[0].fullName}
                        {@const color = invoice.hasPaid ? 'everforest-green' : 'everforest-red'}

                        <div
                            class="flex flex-row items-center justify-evenly w-full text-everforest-black rounded-xl bg-{color} h-[100px] mb-4"
                        >
                            <div class="flex flex-col min-w-[400px]">
                                <span class="text-left">Guest</span>
                                <span class="text-xl font-bold text-left">{guestName}</span>
                            </div>
                            <div class="flex flex-row">
                                <div class="flex flex-col items-center p-4">
                                    <span class="font-bold">Date</span>
                                    <span>{invoice.date}</span>
                                </div>
                                <div class="flex flex-col items-center p-4">
                                    <span class="font-bold">Due Date</span>
                                    <span>{invoice.dueDate}</span>
                                </div>
                                <div class="flex flex-col items-center p-4">
                                    <span class="font-bold">Amount</span>
                                    <span>{amount}</span>
                                </div>
                            </div>
                            <button on:click={() => viewDetail(id)}>
                                <Fa icon={faPrint} class="text-everforest-black/75 text-2xl hover:text-everforest-black/100 hover:duration-200" />
                            </button>
                        </div>
                    {/await}
                {/each}
            {:else}
                <h1 class="text-center font-bold text-xl">Invoice is empty</h1>
            {/if}
        </div>
    </div>
</div>

{#if showDetail}
    {#await getInvoiceDetail(invoiceId) then invoice}
        <Invoice
            id={invoice.id}
            date={invoice.date}
            dueDate={invoice.dueDate}
            fullname={invoice.fullname}
            contact={invoice.contact}
            items={invoice.items}
            duration={invoice.duration}
            roomPrice={invoice.roomPrice}
            bedType={invoice.bedType}
            totalAmount={invoice.totalAmount}
            bind:showDetail
        />
    {/await}
{/if}
