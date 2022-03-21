import React, { useCallback } from 'react';
import { fromLonLat } from 'ol/src/proj';
import 'ol/ol.css';

import { RMap, ROSM } from 'rlayers';
export default function Cluster() {
  const center = fromLonLat([2.364, 48.82]);

  return (
    <>
      <RMap width={'100%'} height={'60vh'} initial={{ center: center, zoom: 11 }}>
        <ROSM />
      </RMap>
    </>
  );
}
