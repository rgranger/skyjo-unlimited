<script>
	import Card from '@smui/card'
	import Button from '@smui/button'

	const handleCreateNewRoom = () => {
		console.log('create new room')
	}

	const handleJoinRoom = () => {
		console.log('join room')
	}
</script>

<svelte:head>
	<title>Skyjo Unlimited</title>
	<meta name="description" content="Skyjo Unlimited" />
</svelte:head>

<section>
	<Card variant="raised" class="gap-3 p-6 ">
		<h1>SKYJO UNLIMITED</h1>
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
