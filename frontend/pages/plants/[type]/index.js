import Link from 'next/link';
import styles from '../../../styles/Button.module.css';

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
        <article className="prose m-5">
            <h2>
                {type} Page {pageNum + 1}/{last_page + 1}
            </h2>

            <Link href={`/plants/${type}?page=0`} passHref>
                <a>
                    <button className={`${styles.btn} ${styles['btn-blue']}`}>first</button>
                </a>
            </Link>

            {pageNum > 0 && (
                <Link href={`/plants/${type}?page=${parseInt(pageNum) - 1}`}>
                    <a>
                        <button className={`${styles.btn} ${styles['btn-blue']}`}>previous</button>
                    </a>
                </Link>
            )}
            {pageNum < last_page && (
                <Link href={`/plants/${type}?page=${parseInt(pageNum) + 1}`}>
                    <a>
                        <button className={`${styles.btn} ${styles['btn-blue']}`}>next</button>
                    </a>
                </Link>
            )}
            <Link href={`/plants/${type}?page=${parseInt(last_page)}`}>
                <a>
                    <button className={`${styles.btn} ${styles['btn-blue']}`}>last</button>
                </a>
            </Link>
            <ul className="list-none">
                {plants.map((item) => (
                    <>
                        <li>
                            <img
                                className="object-contain my-0 mx-2 inline h-6 w-6"
                                src={'/fruit_icons/' + item.type + '.svg'}
                            />
                            <Link
                                href={`/plant/${encodeURIComponent(item.type)}/${encodeURIComponent(
                                    item.name
                                )}`}
                            >
                                {item.name + ' ' + item.type}
                            </Link>
                            {item.marketing_name && (
                                <> (marketed under the {item.marketing_name} brand)</>
                            )}
                        </li>
                    </>
                ))}
            </ul>
        </article>
    );
}
