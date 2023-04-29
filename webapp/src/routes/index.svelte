<script lang="ts">
	import Paper, { Title, Subtitle, Content } from '@smui/paper';
	import Button from '@smui/button';
	import { send } from '$lib/ws';

	const handleCreateRoom = () => {
		send({ type: 'CreateRoom' });
	};

	const handleJoinRoom = () => {
		console.log('join room');
	};
</script>

<main class="h-screen flex flex-col p-10">
	<h1 class="text-8xl text-center">
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

	<div class="flex-1 flex justify-center	items-center">
		<Paper class="flex-1 flex flex-col max-w-lg">
			<Button class="mb-2" on:click={handleCreateRoom} variant="raised">Create Room</Button>
			<Button on:click={handleJoinRoom} variant="raised" disabled>Join Room</Button>
		</Paper>
	</div>
</main>
