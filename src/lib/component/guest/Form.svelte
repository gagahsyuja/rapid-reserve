<script lang="ts">
    export let fullName: string;
    export let nik: string;
    export let roomNumber: string;
    export let checkInDate: string;
    export let checkOutDate: string;
    export let contactInfo: string;
    export let paymentStatus: boolean;
    export let buttonName: string = "Add Guest";
</script>

<form method="POST" on:submit|preventDefault class="flex flex-col p-2">
    <span>Full Name</span>
    <input
        class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2 focus:outline-none"
        type="text"
        bind:value={fullName}
        name="fullname"
        placeholder="Guest's full name"
        required
    />
    <span>NIK</span>
    <input
        class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2"
        type="text"
        bind:value={nik}
        placeholder="Guest's NIK"
        required
    />
    <div class="flex flex-row">
        <div class="flex flex-col items-center">
            <span>Room Number</span>
            <input
                class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2"
                type="text"
                inputmode="numeric"
                maxlength="5"
                bind:value={roomNumber}
                placeholder="Guest's room"
                required
            />
        </div>
        <div class="flex flex-col items-center">
            <span>Check In</span>
            <input
                class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2"
                type="date"
                bind:value={checkInDate}
                placeholder="Check-in Date"
                required
            />
        </div>
        <div class="flex flex-col items-center">
            <span>Check Out</span>
            <input
                class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2"
                type="date"
                bind:value={checkOutDate}
                placeholder="Check-out Date"
                required
            />
        </div>
    </div>
    <span>Contact Info</span>
    <input
        class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2"
        type="tel"
        bind:value={contactInfo}
        placeholder="Guest's contact info"
        required
    />
    <span>Payment Status</span>
    <div class="flex flex-row pb-[7%]">
        <input
            class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2"
            type="radio"
            value={true}
            bind:group={paymentStatus}
            required
        /> Paid
        <input
            class="m-2 bg-[#3A464C] border-4 border-everforest-green rounded-md p-2"
            type="radio"
            value={false}
            bind:group={paymentStatus}
            required
        /> Not Paid
    </div>
    <button
        class="bg-everforest-green p-4 text-everforest-black mx-[120px] rounded-md"
    >
        {buttonName}
    </button>
</form>
