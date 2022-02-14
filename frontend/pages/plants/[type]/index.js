import Link from 'next/link';

export async function getServerSideProps(context) {
    const { type, page } = context.query;
    let pageNum = parseInt(page);
    if (isNaN(pageNum)) {
        pageNum = 0;
    }

    const plants = await fetch(
        `${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/plants/${type}/?page=${pageNum}` // todo - perPage isn't in this API yet
    )
        .then((response) => {
            if (response.status !== 200) {
                return;
            }
            return response.json();
        })
        .catch((error) => {
            console.log(error);
            return;
        });
    return {
        props: {
            plants: plants?.plants || [],
            last_page: plants?.last_page || 0,
            type,
            pageNum
        }
    };
}

export default function Home({ plants, last_page, type, pageNum }) {
    return (
        <div>
            <Link href={`/plants/${type}?page=0`}>first</Link>
            {pageNum > 0 && (
                <Link href={`/plants/${type}?page=${parseInt(pageNum) - 1}`}>previous</Link>
            )}
            {pageNum < last_page && (
                <Link href={`/plants/${type}?page=${parseInt(pageNum) + 1}`}>next</Link>
            )}
            <Link href={`/plants/${type}?page=${parseInt(last_page)}`}>last</Link>
            <ul>
                {plants.map((item) => (
                    <>
                        <li>
                            <img src={'/fruit_icons/' + item.type + '.svg'} height="13" />
                            <Link
                                href={`/plant/${encodeURIComponent(item.type)}/${encodeURIComponent(
                                    item.name
                                )}`}
                            >
                                {item.name + ' ' + item.type}
                            </Link>
                            {item.marketing_name && (
                                <>(marketed under the {item.marketing_name} brand)</>
                            )}
                        </li>
                    </>
                ))}
            </ul>
        </div>
    );
}
