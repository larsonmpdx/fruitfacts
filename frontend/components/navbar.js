import * as React from 'react';
import TextField from '@mui/material/TextField';
import Autocomplete from '@mui/material/Autocomplete';
import throttle from 'lodash/throttle';

// search box: see https://mui.com/components/autocomplete/#search-as-you-type

export default function Home() {
    const [value, setValue] = React.useState(null);
    const [inputValue, setInputValue] = React.useState('');
    const [options, setOptions] = React.useState([]);

    const fetch = React.useMemo(
        () =>
            throttle((request, callback) => {
                console.log('hi');

                // todo - load data
                // 	async function searchPlant(keyword) {
                // const url = `${import.meta.env.VITE_BACKEND_BASE}/api/search/${encodeURIComponent(keyword)}`;

                // const response = await fetch(url);
                // return await response.json();
            }, 400),
        []
    );

    React.useEffect(() => {
        let active = true;
        if (inputValue === '') {
            setOptions(value ? [value] : []);
            return undefined;
        }

        fetch({ input: inputValue }, (results) => {
            if (active) {
                let newOptions = [];

                if (value) {
                    newOptions = [value];
                }

                if (results) {
                    newOptions = [...newOptions, ...results];
                }

                setOptions(newOptions);
            }
        });

        return () => {
            active = false;
        };
    }, [value, inputValue, fetch]);

    return (
        <Autocomplete
            id="search-box"
            sx={{ width: 300 }}
            getOptionLabel={(option) => (typeof option === 'string' ? option : option.description)}
            filterOptions={(x) => x}
            options={options}
            autoComplete
            includeInputInList
            filterSelectedOptions
            value={value}
            onChange={(event, newValue) => {
                setOptions(newValue ? [newValue, ...options] : options);
                setValue(newValue);
            }}
            onInputChange={(event, newInputValue) => {
                setInputValue(newInputValue);
            }}
            renderInput={(params) => <TextField {...params} fullWidth />}
        />
    );
}

/* todo

	fetch(`${import.meta.env.VITE_BACKEND_BASE}/api/checkLogin`, {
		credentials: 'include'
	})
		.then((response) => response.json())
		.then((data) => {
			login.set(data);
		})
		.catch((error) => {
			console.log(error);
		});

	async function logOut() {
		fetch(`${import.meta.env.VITE_BACKEND_BASE}/api/logout`, {
			method: 'POST',
			credentials: 'include'
		})
			.then((response) => {
				if (response.status === 200) {
					login.set({});
				}
				return response.json();
			})
			.catch((error) => {
				console.log(error);
			});
	}


		labelFunction={(plant) => {
			if (plant.marketing_name) {
				return plant.name + ' (' + plant.marketing_name + ') ' + plant.type;
			} else {
				return plant.name + ' ' + plant.type;
			}
		}}



        minCharactersToSearch="3"



        	{#if $login.user}
		logged in as <a href="/user/">{$login.user.name}</a>
		<button type="button" on:click={logOut}>log out</button>
	{:else}
		<a href="/login">log in</a>
	{/if}






    	let selectedPlant;
	$: if (selectedPlant) {
		goto(`/plant?type=${selectedPlant.type}&name=${selectedPlant.name}`);
	}



    */
