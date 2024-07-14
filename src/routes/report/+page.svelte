<script lang="ts">
    import Title from '$lib/component/Title.svelte';
    import Home from '$lib/component/Home.svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from "svelte";

    let reportList: Array<any> = [];
    let filteredReport: Array<any> = [];

    const getAllReports = async () => {

        let reports: string = await invoke('get_all_reports');

        reportList = filteredReport = JSON.parse(reports);
    }

    const getGuestInformation = async (id: number) => {
        
        let guest: string = await invoke('get_guest_information', { guestId: id });

        return JSON.parse(guest);
    }

    onMount(() => {
        getAllReports();
    })
</script>

<Home />
<Title title="Report" />
<div class="w-[60%] m-auto">
    <div class="flex flex-col justify-center">
        <div class="h-[700px] overflow-scroll">
            {#if reportList.length}
                {#each filteredReport as report}
                    {#await getGuestInformation(report.guestId) then guest}
                        {@const guestName = guest[0].fullName}
                        {@const checkInDate = report.checkInDate}
                        {@const actualCheckInDate = report.actualCheckInDate}
                        {@const checkOutDate = report.checkOutDate}
                        {@const actualCheckOutDate = report.actualCheckOutDate}

                        <div
                            class="flex flex-row items-center justify-evenly w-full text-everforest-black rounded-xl bg-everforest-purple h-[100px] mb-4"
                        >
                            <div class="flex flex-col min-w-[50px]">
                                <span class="text-left">Room</span>
                                <span class="text-xl font-bold text-left">{guest[0].roomId}</span>
                            </div>
                            <div class="flex flex-col min-w-[200px]">
                                <span class="text-left">Guest</span>
                                <span class="text-xl font-bold text-left">{guestName}</span>
                            </div>
                            <div class="flex flex-row">
                                <div class="flex flex-col items-center p-4">
                                    <span>Check In Date | Actual</span>
                                    <span class="font-bold">{checkInDate} | {actualCheckInDate ? actualCheckInDate : '-'}</span>
                                </div>
                                <div class="flex flex-col items-center p-4">
                                    <span>Check Out Date | Actual</span>
                                    <span class="font-bold">{checkOutDate} | {actualCheckOutDate ? actualCheckOutDate : '-'}</span>
                                </div>
                            </div>
                        </div>
                    {/await}
                {/each}
            {:else}
                <h1 class="text-center font-bold text-xl">Report is empty</h1>
            {/if}
        </div>
    </div>
</div>
