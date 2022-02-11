export default function Home({ user }) {
    return <>{user && <p>{JSON.stringify(user)}</p>}</>;
}
