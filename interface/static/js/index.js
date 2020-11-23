'use strict'
// TODO: fix all this dom accessing bullshit
// window.onload = () => {

const escapeEl = document.createElement("p");

const clean = (str) => {
    escapeEl.innerText = str;
    return escapeEl.innerHTML;
}

let conn = null

const log = (msg) => {
    div_log.innerHTML += msg + '<br>'
    div_log.scroll(0, div_log.scrollTop + 1000)
}

const connect = () => {
    disconnect()

    const wsUri =
        (window.location.protocol === 'https:' ? 'wss://' : 'ws://') +
        window.location.host +
        '/ws/'

    conn = new WebSocket(wsUri)
    log('Connecting...')

    conn.onopen = function () {
        log('Connected.')
        update_ui()
    }

    conn.onmessage = function (e) {
        log(`<span class="">${clean(e.data)}</span>`)
    }

    conn.onclose = function () {
        log('Disconnected.')
        conn = null

        update_ui()
    }
}

const disconnect = () => {
    if (conn) {
        log('Disconnecting...')
        conn.close()
        conn = null

        update_ui()
    }
}

const update_ui = () => {
    if (!conn) {
        span_status.textContent = 'disconnected'
        btn_connect.textContent = 'Connect'
    } else {
        span_status.textContent = `connected (${conn.protocol})`
        btn_connect.textContent = 'Disconnect'
    }
}

const cmd = (n) => {
    if (!conn) return
    conn.send("scmd" + n)
}

btn_connect.onclick = () => {
    if (!conn) {
        connect()
    } else {
        disconnect()
    }

    update_ui()
}

btn_send.onclick = () => {
    if (!conn) return

    const text = input_text.value
    log(`<span class="green">&gt; ${clean(text)}</span>`)
    conn.send("rcon" + text)

    input_text.value = ''
    input_text.focus()
}

input_text.onkeyup = (e) => {
    if (e.key === 'Enter') {
        btn_send.click()
    }
}

document.getElementById("btn_start").onclick = () => { cmd(0) }
document.getElementById("btn_stop").onclick = () => { cmd(1) }
// }