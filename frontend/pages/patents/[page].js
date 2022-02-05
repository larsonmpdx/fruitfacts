import Link from 'next/link';
import {formatPatentDate} from '../../components/functions'

export async function getServerSideProps(context) {
    const { page } = context.query;
    let pageNum = parseInt(page);
    if (pageNum == NaN) {
        pageNum = 0;
    }

    const patent_list = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/patents?perPage=50&page=${pageNum}`)
        .then((response) => {
            if (response.status !== 200) {
                return {};
            }
            return response.json();
        })
        .catch((error) => {
            console.log(error);
            return {};
        });

    return {
        props: {
            patent_list,
            pageNum
        }
    };
}

export default function Home({ patent_list, pageNum }) {
    return (
        <div>
            <Link
                    href={`/patents/${parseInt(pageNum)-1}`}
                >
                    previous
            </Link>
            <Link
                    href={`/patents/${parseInt(pageNum)+1}`}
                >
                    next
            </Link>
            <ul>
                        {patent_list.map((item) => (
                            <>
                                <li>
                                <Link
                                    href={`/plant/${encodeURIComponent(
                                        item.type
                                    )}/${encodeURIComponent(item.name)}`}
                                >
                                    {item.name + ' ' + item.type}
                                </Link>
                                {item.marketing_name && <>(marketed as {item.marketing_name})</>} {formatPatentDate(item.uspp_expiration)}
                                </li>
                            </>
                        ))}
                    </ul>
        </div>
    );
}
