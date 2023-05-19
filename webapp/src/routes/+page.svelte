<script>
	import Card from '@smui/card';
	import Button from '@smui/button';

	import { socket } from '$lib/ws';

	const handleCreateNewRoom = () => {
		socket?.send('create a room');
	};

	const handleJoinRoom = () => {
		socket?.send('join room');
	};
</script>

<svelte:head>
	<title>Skyjo Unlimited</title>
	<meta name="description" content="Skyjo Unlimited" />
</svelte:head>

<section>
	<Card variant="raised" class="gap-3 p-6 ">
		<h1>
			<span class="text-red-500">S</span>
			<span class="text-yellow-500">K</span>
			<span class="text-green-500">Y</span>
			<span class="text-cyan-500">J</span>
			<span class="text-blue-500">O</span>
			<span>&nbsp;</span>
			<span class="text-red-500">U</span>
			<span class="text-yellow-500">N</span>
			<span class="text-green-500">L</span>
			<span class="text-cyan-500">I</span>
			<span class="text-blue-500">M</span>
			<span class="text-red-500">I</span>
			<span class="text-yellow-500">T</span>
			<span class="text-green-500">E</span>
			<span class="text-cyan-500">D</span>
		</h1>
		<Button on:click={handleCreateNewRoom} variant="raised">Create New Room</Button>
		<Button on:click={handleJoinRoom} variant="outlined" color="secondary">Join Room</Button>
	</Card>
</section>

<style>
	section {
		flex: 0.9;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}
</style>
