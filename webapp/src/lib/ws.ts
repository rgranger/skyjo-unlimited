import { browser } from '$app/environment'

export let socket: WebSocket | null = null

if (browser) {
    socket = new WebSocket('ws://localhost:4000')

    socket.onopen = () => {
        console.log('open')
    }
    
    socket.onmessage = (message) => {
        console.log('message', message)
    }
    
    socket.onclose = () => {
        console.log('close')
    }
    
    socket.onerror = (error) => {
        console.log('error')
    }
}
