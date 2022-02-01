// ideally we could have one file to render both directory listings and collections like this:
// for a path ending in "/" like /collections/Oregon/ treat this like a folder listing
// for a path ending in something else like /collections/Oregon/USDA-OSU Releases, treat it as an individual collection display

// but (Jan 2022) this isn't possible in next.js because of dumb redirect rules (all URLs get rewritten to either end in '/' or not)
// see https://github.com/vercel/next.js/discussions/23988

// so we have this split between /dirs/[...path].js (directory listings) and /collections/[...path].js (individual collections)

export async function getServerSideProps(context) {
	const { path } = context.query;
	const data = await fetch(`${process.env.BACKEND_BASE}/api/collections/${path.join('/')}`) // no trailing slash - individual collection
		.then((response) => {
			if (response.status !== 200) {
				return [];
			}
			return response.json();
		})
		.catch((error) => {
			console.log(error);
			return [];
		});

	return {
		props: {
			data
		}
	};
}

export default function Home({ data }) {
	return (
		<div>
			{/* single collection */}
			{data.collection && (
				<>
					<p>
						{data.collection.title}
						{data.collection.url && <a href={data.collection.url}>[ref]</a>}
					</p>
					<h1>Locations</h1>
					<ul>
						{data.locations.map((location) => (
							<li>{location.location_name}</li>
						))}
					</ul>
					<h1>Plants</h1>
					<ul>
						{data.items.map((item) => (
							<li>
								<a
									href={`/plant/${encodeURIComponent(item.type)}/${encodeURIComponent(item.name)}`}
								>
									{item.name} {item.type}
								</a>

								{item.marketing_name && <>(marketed as {item.marketing_name})</>}
							</li>
						))}
					</ul>
				</>
			)}
		</div>
	);
}
