<script lang="ts">
    import { clickoutside } from '@svelte-put/clickoutside';

    export let showDetail: boolean = false;
    export let id: number = 14259;
    export let date: string = "2004-06-10";
    export let dueDate: string = "2004-06-10";
    export let fullname: string = "Rapid Reserve"
    export let contact: string = '';
    export let items: any = null;
    export let duration: number = 0;
    export let roomPrice: number = 0;
    export let bedType: string = 'King';
    export let totalAmount: number = 0;
</script>

<div class="fixed w-full h-full bg-everforest-black/80 top-0 left-0 z-50">
    <div use:clickoutside on:clickoutside={() => showDetail = false} class="bg-everforest-white w-[800px] h-auto m-auto p-4 rounded-lg mt-[10%]">
        <h1 class="text-3xl text-center font-bold bg-everforest-black text-everforest-white p-4">INVOICE</h1>
        <div class="flex flex-col float-left">
            <h1 class="text-everforest-black font-bold text-3xl pt-5 mt-6 px-8">{fullname.toUpperCase()}</h1>
            <span class="text-everforest-black text-xl pb-8 mt-0 px-8">{contact}</span>
        </div>
        <table class="table-fixed float-right text-everforest-black w-[35%] border-0 border-everforest-black border-separate my-6 text-left">
            <tr>
                <td class="relative px-2 font-bold">
                    <span>INVOICE #</span>
                </td>
                <td class="relative bg-everforest-black/20 px-4 py-1 rounded-md">
                    <span>{id}</span>
                </td>
            </tr>
            <tr>
                <td class="relative px-2 font-bold">
                    <span>DATE</span>
                </td>
                <td class="relative bg-everforest-black/20 px-4 py-1 rounded-md">
                    <span>{date}</span>
                </td>
            </tr>
            <tr>
                <td class="relative px-2 font-bold">
                    <span>DUE DATE</span>
                </td>
                <td class="relative bg-everforest-black/20 px-4 py-1 rounded-md">
                    <span>{dueDate}</span>
                </td>
            </tr>
        </table>
        <table class="text-everforest-black w-full text-center bg-everforest-black/5 shadow-md shadow-black mt-10">
            <tr class="bg-everforest-black/30">
                <th class="px-4 py-2">ITEM</th>
                <th class="px-4 py-2">RATE</th>
                <th class="px-4 py-2">QUANTITY</th>
                <th class="px-6 py-2">AMOUNT</th>
            </tr>
            <tr class="border-b border-everforest-black">
                <td class="p-2">ROOM W/ {bedType.toUpperCase()}</td>
                <td class="p-2">IDR {roomPrice}</td>
                <td class="p-2">{duration}</td>
                <td class="p-2">IDR {roomPrice * duration}</td>
            </tr>
            {#each items.items as item, i}
                {@const price = items.price[i]}
                {@const amount = items.amount[i]}
                <tr class="border-b border-everforest-black">
                    <td class="p-2">{item.toUpperCase()} SERVICE</td>
                    <td class="p-2">IDR {price}</td>
                    <td class="p-2">{amount}</td>
                    <td class="p-2">IDR {price * amount}</td>
                </tr>
            {/each}
            <tr>
                <td colspan=3 class="font-bold bg-everforest-black/10 py-3">GRAND TOTAL</td>
                <td class="font-bold bg-everforest-black/10">IDR {totalAmount}</td>
            </tr>
        </table>
        <h1 class="text-center text-everforest-black/30 font-semibold py-4 mt-4">Rapid Reserve</h1>
    </div>
</div>
