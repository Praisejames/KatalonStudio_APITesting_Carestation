<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Senior Request</name>
   <tag></tag>
   <elementGuidId>a09f9f51-34bb-435c-a2ee-517e21e23e16</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;address\&quot;: \&quot;35 Onideure Area\&quot;,\n    \&quot;avatar_url\&quot;: \&quot;https://google.com\&quot;,\n    \&quot;city\&quot;: \&quot;Gbagi\&quot;,\n    \&quot;country\&quot;: \&quot;Nigeria\&quot;,\n    \&quot;date_of_birth\&quot;: \&quot;2020-10-08\&quot;,\n    \&quot;email\&quot;: \&quot;kunlemartins@gmail.com\&quot;,\n    \&quot;firstname\&quot;: \&quot;Kunle\&quot;,\n    \&quot;gender\&quot;: 0,\n    \&quot;lastname\&quot;: \&quot;Martins\&quot;,\n    \&quot;nickname\&quot;: \&quot;Kunle\&quot;,\n    \&quot;personal_id\&quot;: \&quot;4343343\&quot;,\n    \&quot;phone\&quot;: \&quot;5545454\&quot;,\n    \&quot;postal_code\&quot;: \&quot;2EDRF\&quot;,\n    \&quot;province\&quot;: \&quot;Oyo\&quot;,\n    \&quot;status\&quot;: 1,\n    \&quot;user_id\&quot;: 1,\n    \&quot;title\&quot;:\&quot;Mr\&quot;,\n    \&quot;user_type\&quot;: 1\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>008a5a35-784d-4922-96da-aa14d6bef8b8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjJiOGFiMGZiLTc3MWMtNDU5ZS04OGIwLTM2MjdhNDE4OTc4MiIsInVzZXJfaWQiOjMzLCJ1c2VybmFtZSI6IktheXN0aWNrcyIsImVtYWlsIjoia3N0aWNrc0BnbWFpbC5jb20iLCJ1c2VyX3R5cGUiOjEsImlzc3VlZF9hdCI6IjIwMjItMDUtMzBUMTE6MzI6MDguNTkzODc0ODkzWiIsImV4cGlyZWRfYXQiOiIyMDIyLTA1LTMwVDExOjQ3OjA4LjU5Mzg3NTA5N1oifQ.0YCxIne_jN22JlIf4dXgAupvcjsYN9VQpd89sJuzZCw</value>
      <webElementGuid>170e6ef8-5a70-4031-86ff-0609f0907a1e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://carestationrest.herokuapp.com/api/v1/seniors</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
