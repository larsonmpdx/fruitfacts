import Link from 'next/link';
import { formatPatentDate } from '../../components/functions';

export async function getServerSideProps(context) {
    const { page } = context.query;
    let pageNum = parseInt(page);
    if (isNaN(pageNum)) {
        pageNum = 0;
    }

    const patent_info = await fetch(
        `${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/patents?perPage=50&page=${pageNum}`
    )
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
            patent_info,
            pageNum
        }
    };
}

export default function Home({ patent_info, pageNum }) {
    return (
        <div>
            <Link href={`/patents/${parseInt(pageNum) - 1}`}>previous</Link>
            <Link href={`/patents/${parseInt(pageNum) + 1}`}>next</Link>
            <ul>
                {patent_info.patents.map((item) => (
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
                            )}{' '}
                            {formatPatentDate(item.uspp_expiration, item.uspp_expiration_estimated)}
                        </li>
                    </>
                ))}
            </ul>
        </div>
    );
}
