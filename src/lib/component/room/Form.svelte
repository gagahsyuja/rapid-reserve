<script lang="ts">
    export let roomNumber: string;
    export let bedType: string;
    export let price: string;
    export let occupied: boolean = false;
    export let buttonName: string = "Add Room";
</script>

<form method="POST" on:submit|preventDefault class="flex flex-col p-2">
    <span>Room Number</span>
    <input
        class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2 focus:outline-none"
        type="text"
        inputmode="numeric"
        pattern="[0-9]+"
        bind:value={roomNumber}
        title="Has to be a number"
        name="fullname"
        placeholder="Room's number"
        required
    />
    <span>Bed Type</span>
    <select
        class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2"
        bind:value={bedType}
        required
    >
        <option>Single</option>
        <option>Twin</option>
        <option>Double</option>
        <option>King</option>
    </select>
    <span>Price</span>
    <input
        class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2"
        type="text"
        bind:value={price}
        placeholder="Room's price"
        required
    />
    <span>Room Status</span>
    <div class="flex flex-row pb-[7%]">
        <input
            class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2"
            type="radio"
            value={true}
            bind:group={occupied}
            required
        /> Occupied
        <input
            class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2"
            type="radio"
            value={false}
            bind:group={occupied}
            required
        /> Not Occupied
    </div>
    <button
        class="bg-everforest-green p-4 text-everforest-black mx-[120px] mt-[30px] rounded-md"
    >
        {buttonName}
    </button>
</form>
