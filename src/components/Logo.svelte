<script lang="ts">
  let { foreground = "var(--foreground)", background = "rgba(0, 0, 0, 0)", class: className = "" } = $props();
</script>

<svg width="512" height="512" viewBox="0 0 512 512" fill="none" xmlns="http://www.w3.org/2000/svg" class={className}>
<rect width="512" height="512" fill={background}/>

<mask id="mask0_0_1" style="mask-type:alpha" maskUnits="userSpaceOnUse" x="101" y="321" width="310" height="130">
<rect x="101" y="321" width="310" height="130" fill="white"/>
</mask>
<g mask="url(#mask0_0_1)">
<circle cx="191" cy="321" r="65" stroke={foreground} stroke-width="50" stroke-miterlimit="16"/>
</g>

<mask id="mask1_0_1" style="mask-type:alpha" maskUnits="userSpaceOnUse" x="101" y="61" width="310" height="130">
<rect x="101" y="61" width="310" height="130" fill="white"/>
</mask>
<g mask="url(#mask1_0_1)">
<circle cx="321" cy="191" r="65" stroke={foreground} stroke-width="50" stroke-miterlimit="16"/>
</g>

<mask id="mask2_0_1" style="mask-type:alpha" maskUnits="userSpaceOnUse" x="101" y="191" width="310" height="130">
<rect x="101" y="191" width="310" height="130" fill="white"/>
</mask>
<g mask="url(#mask2_0_1)">
<rect x="126" y="166" width="130" height="180" stroke={foreground} stroke-width="50"/>
<rect x="256" y="166" width="130" height="180" stroke={foreground} stroke-width="50"/>
</g>

</svg>


