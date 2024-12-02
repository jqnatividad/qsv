

Incident Report #{{Unique_Key}}

Created: {{Created_Date}}
{% if Closed_Date %}Closed: {{Closed_Date}}{% endif %}
Status: {{Status}}

Agency: {{Agency}} ({{Agency_Name}})
Complaint: {{Complaint_Type}}
Description: {{Descriptor}}

Location Details:
{% if Location_Type %}Type: {{Location_Type}}{% endif %}
{% if Incident_Address %}Address: {{Incident_Address}}{% endif %}
{% if Street_Name %}Street: {{Street_Name}}{% endif %}
{% if Cross_Street_1 and Cross_Street_2 %}Between: {{Cross_Street_1}} and {{Cross_Street_2}}{% endif %}
{% if City %}City: {{City}}{% endif %}
Borough: {{Borough}}
{% if Incident_Zip %}ZIP: {{Incident_Zip}}{% endif %}
{% if Latitude and Longitude %}Coordinates: {{Latitude}}, {{Longitude}}{% endif %}

{% if Resolution_Description %}
Resolution:
{{Resolution_Description}}
{% endif %}

{% if Community_Board %}Community Board: {{Community_Board}}{% endif %}
Channel: {{Open_Data_Channel_Type}}

-----------------------------------------------------------------------

