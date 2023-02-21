import React from 'react';
import Button from '../components/button';
import ConfirmModal from '../components/confirmModal';

export default function Home({ setContributingLinks, setErrorMessage }) {
  React.useEffect(() => {
    setContributingLinks([{ link: `/frontend/pages/user.js`, description: `user.js` }]);
  }, []);

  const [fullUser, setFullUser] = React.useState(null);
  const [deleteModalVisible, setDeleteModalVisible] = React.useState(false);

  const deleteUser = async () => {
    // todo
  };

  React.useEffect(() => {
    // todo - this is too many lines for what it does. simplify (and other occurrences)
    const fetchData = async () => {
      const data = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/getFullUser`, {
        credentials: 'include'
      })
        .then((response) => {
          if (response.status === 200) {
            return response.json();
          } else {
            setErrorMessage("couldn't get full user");
            return null;
          }
        })
        .catch((error) => {
          setErrorMessage(`couldn't get full user: ${error.message}`);
          console.log(error);
          return null;
        });

      setFullUser(data);
    };

    fetchData();
  }, []);

  return (
    <>
      <ConfirmModal
        enabled={deleteModalVisible}
        okFunction={deleteUser}
        cancelFunction={() => {
          setDeleteModalVisible(false);
        }}
        title="Delete Account?"
        text="this will delete your account and all of your lists"
      ></ConfirmModal>
      <div className="w-3/5">
        <div className="rounded-lg border bg-indigo-800 p-10 font-bold text-white shadow-lg">
          {fullUser && (
            <pre className="whitespace-pre-wrap break-all">{JSON.stringify(fullUser, null, 2)}</pre>
          )}
        </div>
        <Button
          enabled={true}
          onClick={() => {
            setDeleteModalVisible(true);
          }}
          color="red"
          label="delete my account"
        />
      </div>
    </>
  );
}
