<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from "svelte";

    let searchKeyword: string = '';

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

    onMount(async () => {
        getAllReports().then(() => console.log(reportList));
    })
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
            {#if reportList.length}
                {#each filteredReport as report}
                    {#await getGuestInformation(report.guestId) then guest}
                        {@const guestName = guest[0].fullName}
                        {@const checkInDate = report.checkInDate}
                        {@const checkOutDate = report.checkOutDate}
                        {@const color = 'everforest-green'}

                        <button
                            class="flex flex-row items-center justify-center w-full text-everforest-black rounded-xl bg-{color} hover:bg-everforest-red h-[100px] mb-4"
                        >
                            <div class="flex flex-col items-center">
                                <span>Guest</span>
                                <span class="text-xl">{guestName}</span>
                            </div>
                            <div class="flex flex-row">
                                <div class="flex flex-col items-center p-4">
                                    <span>Check In date</span>
                                    <span>{checkInDate}</span>
                                </div>
                                <div class="flex flex-col items-center p-4">
                                    <span>Check Out Date</span>
                                    <span>{checkOutDate}</span>
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
