// This worker is completely vibecoded. I have no idea what I'm doing with workers and stuff.
export default {
	async fetch(request, env, ctx): Promise<Response> {
		if (request.method === 'OPTIONS') {
			return new Response(null, {
				status: 204,
				headers: {
					'Access-Control-Allow-Origin': '*',
					'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',
					'Access-Control-Allow-Headers': '*',
					'Access-Control-Max-Age': '86400',
				},
			});
		}

		const url = new URL(request.url);
		const target = url.searchParams.get('url');
		if (!target) return new Response('Missing ?url=', { status: 400 });

		const isWebSocket = request.headers.get('Upgrade') === 'websocket';

		if (isWebSocket) {
			const webSocketPair = new WebSocketPair();
			const [client, server] = Object.values(webSocketPair);

			const upstreamHeaders = new Headers(request.headers);
			upstreamHeaders.set('Upgrade', 'websocket');
			upstreamHeaders.set('Connection', 'Upgrade');
			upstreamHeaders.delete('host');

			let upstream;
			try {
				const upstreamUrl = target.replace(/^wss:\/\//i, 'https://').replace(/^ws:\/\//i, 'http://');

				const upstreamResp = await fetch(upstreamUrl, {
					headers: upstreamHeaders,
				});

				if (!upstreamResp.webSocket) {
					server.accept();
					server.close(1011, `Upstream did not upgrade: ${upstreamResp.status}`);
					return new Response(null, { status: 101, webSocket: client });
				}
				upstream = upstreamResp.webSocket;
			} catch (e) {
				return new Response(`Upstream connect failed: ${e}`, { status: 502 });
			}

			server.accept();
			upstream.accept();

			server.addEventListener('message', (e) => upstream.send(e.data));
			server.addEventListener('close', (e) => upstream.close(e.code, e.reason));
			upstream.addEventListener('message', (e) => server.send(e.data));
			upstream.addEventListener('close', (e) => server.close(e.code, e.reason));
			upstream.addEventListener('error', (e) => server.close(1011, 'Upstream error'));

			return new Response(null, { status: 101, webSocket: client });
		}

		const targetUrl = new URL(target);
		if (['localhost', '127.0.0.1', '0.0.0.0'].includes(targetUrl.hostname)) {
			return new Response('Forbidden', { status: 403 });
		}

		const proxyRequest = new Request(target, {
			method: request.method,
			headers: (() => {
				const h = new Headers(request.headers);
				h.delete('origin');
				h.delete('referer');
				return h;
			})(),
			body: ['GET', 'HEAD'].includes(request.method) ? undefined : request.body,
		});

		const response = await fetch(proxyRequest);
		const corsResponse = new Response(response.body, response);
		corsResponse.headers.set('Access-Control-Allow-Origin', '*');
		corsResponse.headers.set('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS');
		corsResponse.headers.set('Access-Control-Allow-Headers', '*');
		return corsResponse;
	},
} satisfies ExportedHandler<Env>;
