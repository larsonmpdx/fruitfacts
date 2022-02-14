import 'bulma/css/bulma.min.css';
import Layout from '../components/layout';

export default function MyApp({ Component, pageProps }) {
    return (
        <Layout>
            <Component {...pageProps} />
        </Layout>
    );
}
