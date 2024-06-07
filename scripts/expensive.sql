SELECT
    A.Agency,
    A.Borough,
    COUNT(*) AS total_incidents,
    SUM(
        CASE
            WHEN A."Complaint Type" LIKE 'Noise%' THEN 1
            ELSE 0
        END
    ) AS noise_related_incidents,
    SUM(
        CASE
            WHEN A.Status = 'Closed' THEN 1
            ELSE 0
        END
    ) AS closed_incidents,
    SUM(
        CASE
            WHEN A.Status != 'Closed' THEN 1
            ELSE 0
        END
    ) AS open_incidents,
    SUM(
        CASE
            WHEN POSITION('Water' IN A."Complaint Type") > 0 THEN 1
            ELSE 0
        END
    ) AS water_related_incidents,
    MAX(LENGTH (A."Complaint Type")) AS max_complaint_type_length,
    SUM(
        CASE
            WHEN UPPER(A."Complaint Type") = UPPER(A."Complaint Type") 
            THEN LENGTH (A."Complaint Type")
            ELSE 0
        END
    ) AS sum_complaint_type_lengths,
    COUNT(DISTINCT A."Complaint Type") AS distinct_complaint_types
FROM
    read_csv ('NYC_311_SR_2010-2020-sample-1M.csv') A
GROUP BY
    A.Agency,
    A.Borough
ORDER BY
    total_incidents DESC;
