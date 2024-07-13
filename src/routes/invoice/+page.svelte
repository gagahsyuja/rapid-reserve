<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from "svelte";

    let invoiceList: Array<any> = [];

    let searchKeyword: string = '';

    const getAllInvoices = async () => {

        let invoices: string = await invoke('get_all_invoices');

        invoiceList =  JSON.parse(invoices);
    }

    const getGuestInformation = async (id: number) => {
        
        let guest: string = await invoke('get_guest_information', { guestId: id });

        return JSON.parse(guest);
    }

    onMount(() => {
        getAllInvoices().then(() => console.log(invoiceList));
    });
</script>

<div class="p-8">
    <div class="flex flex-col justify-center">
        <input
            class="bg-everforest-black border-4 border-everforest-green shadow-md focus:outline-none w-[80%] h-[50px] text-everforest-white placeholder:text-everforest-white p-4 rounded-3xl mt-8 mb-6 self-center"
            type="text"
            placeholder="Search name..."
            bind:value={searchKeyword}
        />

        <div class="h-[700px] overflow-scroll">
            {#if invoiceList.length}
                {#each invoiceList as invoice}
                    {#await getGuestInformation(invoice.guestId) then guest}
                        {@const amount = invoice.amountToPay}
                        {@const guestName = guest[0].fullName}
                        {@const color = invoice.hasPaid ? 'everforest-green' : 'everforest-red'}

                        <button
                            class="flex flex-row items-center justify-center w-full text-everforest-black rounded-xl bg-{color} hover:bg-everforest-red h-[100px] mb-4"
                        >
                            <div class="flex flex-col items-center">
                                <span>Guest</span>
                                <span class="text-xl">{guestName}</span>
                            </div>
                            <div class="flex flex-row">
                                <div class="flex flex-col items-center p-4">
                                    <span>Date</span>
                                    <span>{invoice.date}</span>
                                </div>
                                <div class="flex flex-col items-center p-4">
                                    <span>Due Date</span>
                                    <span>{invoice.dueDate}</span>
                                </div>
                                <div class="flex flex-col items-center p-4">
                                    <span>Amount</span>
                                    <span>{amount}</span>
                                </div>
                            </div>
                        </button>
                    {/await}
                {/each}
            {:else}
                <span>Guest is empty</span>
            {/if}
        </div>
    </div>
</div>
