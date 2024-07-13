<script lang="ts">
    export let greenIdentifier: string;
    export let redIdentifier: string;
</script>

<div class="fixed flex flex-row items-center justify-evenly bottom-0 left-0 w-full h-[100px] bg-[#293136]">
    <div class="flex flex-row items-center bg-everforest-black py-2 px-4 rounded-lg">
        <div class="w-[50px] h-[50px] bg-everforest-green rounded-md"></div>
        <span class="font-bold p-4">{greenIdentifier}</span>
    </div>
    <div class="flex flex-row items-center bg-everforest-black py-2 px-4 rounded-lg">
        <div class="w-[50px] h-[50px] bg-everforest-red rounded-md"></div>
        <span class="font-bold p-4">{redIdentifier}</span>
    </div>
</div>
