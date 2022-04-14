// display an error

export default function Home({ errorMessage }) {
  if (errorMessage) {
    return (
      <>
        <p>{errorMessage}</p>
      </>
    );
  }
  return <></>;
}
