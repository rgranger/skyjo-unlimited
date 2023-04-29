let ws: WebSocket | null = null

const url = 'ws://localhost:4000'

export function init() {
    ws = new WebSocket(url)
    let reconnectInterval: NodeJS.Timer | null = null

    ws.addEventListener('open', () => {
        console.log('ws connected')

        if (reconnectInterval !== null) {
            clearInterval(reconnectInterval)
            reconnectInterval = null
        }
    })
    
    ws.addEventListener('close', () => {
        console.log('ws disconnected')

        reconnectInterval = setInterval(() => {
            ws = new WebSocket(url)
        }, 3000)
    })
    
    ws.addEventListener('error', (e) => {
        console.log('ws error:', e)
    })
    
    ws.addEventListener('message', (message) => {
        console.log('ws message', message)
    })
}

interface CreateRoom {
    type: "CreateRoom"
}

interface JoinRoom {
    type: "JoinRoom",
    id: string
}

type Request = CreateRoom | JoinRoom

export function send(request: Request) {
    if (ws !== null && ws.readyState === ws.OPEN) {
        ws.send(JSON.stringify(request))
    }
}
