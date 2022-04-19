// after the backend completes an oauth login and gets a redirect back to the backend
// it will check the login info against the database. if no account is found it redirects here
// so the user is prompted to create an account (or not)
import React from 'react'
import Link from 'next/link'
import ButtonLink from '../components/buttonLink'
import Button from '../components/button'

export default function Home ({ setErrorMessage }) {
  const [user, setUser] = React.useState()
  const [clicked, setClicked] = React.useState()

  const createAccount = async () => {
    if (clicked) {
      return
    }
    setClicked(true)

    const user = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/createAccount`, {
      credentials: 'include'
    })
      .then(response => {
        if (response.status !== 200) {
          setErrorMessage('failed creating user')
          return { failed: response.status }
        }
        return response.json()
      })
      .catch(error => {
        setErrorMessage(`failed creating user: ${error.message}`)
        console.log(error)
        return { failed: error }
      })
    setUser(user)
  }

  return (
    <div class='flex items-center justify-center h-screen'>
      <div class='columns-1'>
        {!user && (
          <>
            <div class='bg-indigo-800 text-white font-bold rounded-lg border shadow-lg p-10'>
              <p>
                external login was successful but no {process.env.NEXT_PUBLIC_SITE_NAME} account was
                found. create one?
              </p>
            </div>
            <div class='bg-indigo-800 text-white font-bold rounded-lg border shadow-lg p-10'>
              <Button
                enabled={!clicked}
                onClick={async () => {
                  await createAccount()
                }}
                class='w-full h-12 px-6 text-indigo-100 transition-colors duration-150 bg-indigo-700 rounded-lg focus:shadow-outline hover:bg-indigo-800'
                label='create account'
              />
            </div>
          </>
        )}
        {user && (
          <div class='bg-indigo-800 text-white font-bold rounded-lg border shadow-lg p-10'>
            {user?.failed ? (
              <p>failed creating account: {user.failed}</p>
            ) : (
              <pre>account created: {JSON.stringify(user, null, 2)}</pre>
            )}
          </div>
        )}
        <div class='bg-indigo-800 text-white font-bold rounded-lg border shadow-lg p-10'>
          <ButtonLink href='/' label={`Back to ${process.env.NEXT_PUBLIC_SITE_NAME}`} 
          class='w-full h-12 px-6 text-indigo-100 transition-colors duration-150 bg-indigo-700 rounded-lg focus:shadow-outline hover:bg-indigo-800'
          />
        </div>
      </div>
    </div>
  )
}
