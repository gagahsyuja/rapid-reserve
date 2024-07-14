<script lang="ts">
    import Fa from "svelte-fa";
    import { faCheckCircle, faTriangleExclamation } from '@fortawesome/free-solid-svg-icons';

    export let success: boolean = true;
    export let message: string = 'Success';
</script>

<div class="fixed top-0 left-0 flex justify-center items-center w-full bg-everforest-black/90 z-50 h-full">
    {#if success}
        <div class="flex flex-col justify-center p-4 w-[28%] m-auto text-center h-[600px] bg-everforest-black border-4 border-everforest-green rounded-3xl">
            <Fa icon={faCheckCircle} class="text-everforest-green text-8xl mb-[50px]" />
            <h1 class="text-3xl text-everforest-green self-center p-4 font-bold">{message}</h1>
            <button
                on:click
                class="bg-everforest-green/100 hover:bg-everforest-green/80 text-xl text-everforest-black rounded-xl py-2 w-[100px] self-center"
            >
                Close
            </button>
        </div>
    {:else}
        <div class="flex flex-col justify-center p-4 w-[28%] text-center m-auto h-[600px] bg-everforest-black border-4 border-everforest-yellow rounded-3xl">
            <Fa icon={faTriangleExclamation} class="text-everforest-yellow text-8xl mb-[50px]" />
            <h1 class="text-3xl text-everforest-yellow self-center p-4 font-bold">{message}</h1>
            <button
                on:click
                class="bg-everforest-yellow/100 hover:bg-everforest-yellow/80 text-xl text-everforest-black rounded-xl py-2 w-[100px] self-center"
            >
                Close
            </button>
        </div>
    {/if}
</div>
